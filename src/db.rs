use sqlx::{query, query_as, PgConnection, QueryBuilder, Type};
use std::{error::Error, fmt::Display};
use time::{format_description::well_known::Iso8601, OffsetDateTime, PrimitiveDateTime};

pub struct Match {
    pub id: i32,
    pub match_start: PrimitiveDateTime,
    pub set_start: PrimitiveDateTime,
    pub team_a: String,
    pub team_b: String,
    pub swapped: bool,
    pub result: Vec<i32>,
    pub set_results_a: Vec<i32>,
    pub set_results_b: Vec<i32>,
    pub status: MatchStatus,
}

#[derive(Type, PartialEq, Eq, Debug)]
#[sqlx(type_name = "match_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MatchStatus {
    Finished,
    Planned,
    InProgress,
}

pub async fn match_exists(conn: &mut PgConnection, match_id: i32) -> bool {
    query!(
        r#"SELECT COUNT(*) as "count!" FROM matches WHERE id=$1"#,
        match_id
    )
    .fetch_one(conn)
    .await
    .unwrap()
    .count
        > 0
}

pub async fn get_matches(conn: &mut PgConnection) -> Vec<Match> {
    query_as!(
        Match,
        r#"SELECT id, match_start, set_start, team_a, team_b, swapped, result, set_results_a, set_results_b, status "status: MatchStatus" FROM matches"#
    )
    .fetch_all(conn)
    .await
    .unwrap()
}

pub async fn get_match(conn: &mut PgConnection, match_id: i32) -> Match {
    query_as!(
        Match,
        r#"SELECT id, match_start, set_start, team_a, team_b, swapped, result, set_results_a, set_results_b, status "status: MatchStatus" FROM matches WHERE id=$1"#,match_id
    )
    .fetch_one(conn)
    .await
    .unwrap()
}

pub async fn swap_teams(conn: &mut PgConnection, match_id: i32) -> bool {
    query!(
        "UPDATE matches SET swapped=not swapped WHERE id=$1 AND status='IN_PROGRESS'",
        match_id
    )
    .execute(conn)
    .await
    .unwrap()
    .rows_affected()
        > 0
}

#[derive(Debug)]
pub enum MatchAddError {
    TeamNameEmpty,
    TeamNameTooLong,
    DuplicateTeamName,
    PastDate,
    IncorrectDateFormat,
}

impl Display for MatchAddError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str( match self {
                MatchAddError::TeamNameEmpty => "Team names cannot be empty",
                MatchAddError::DuplicateTeamName => "Team names cannot be the same" ,
                MatchAddError::PastDate => "Past dates are not allowed",
                MatchAddError::IncorrectDateFormat => "Incorrect date format. Date has to be a valid ISO8601 timestamp. Leave empty to use current time",
                MatchAddError::TeamNameTooLong => "Team name can't be longer than 50 characters",
            }
        )
    }
}

impl Error for MatchAddError {}

pub async fn add_match(
    conn: &mut PgConnection,
    team_a_name: &str,
    team_b_name: &str,
    match_date: &str,
) -> Result<Match, MatchAddError> {
    let team_a_name = team_a_name.trim();
    let team_b_name = team_b_name.trim();
    if team_a_name.is_empty() || team_b_name.is_empty() {
        return Err(MatchAddError::TeamNameEmpty);
    }
    if team_a_name.chars().count() > 50 || team_b_name.chars().count() > 50 {
        return Err(MatchAddError::TeamNameTooLong);
    }
    if team_a_name == team_b_name {
        return Err(MatchAddError::DuplicateTeamName);
    }
    if match_date.trim().is_empty() {
        Ok(query_as!(
                    Match,
                    r#"INSERT INTO matches(team_a, team_b) VALUES($1, $2) RETURNING id, match_start, set_start, team_a, team_b, swapped, result, set_results_a, set_results_b, status "status: MatchStatus""#,
                    team_a_name,
                    team_b_name
                )
                .fetch_one(conn)
                .await.unwrap())
    } else if let Ok(date) = PrimitiveDateTime::parse(&match_date, &Iso8601::DEFAULT) {
        if date.assume_utc() > OffsetDateTime::now_utc() {
            Ok(query_as!(
                            Match,
                            r#"INSERT INTO matches(status, match_start, set_start, team_a, team_b) VALUES('PLANNED', $1, $1, $2, $3) RETURNING id, match_start, set_start, team_a, team_b, swapped, result, set_results_a, set_results_b, status "status: MatchStatus""#,
                            date,
                            team_a_name,
                            team_b_name
                        )
                        .fetch_one(conn)
                        .await.unwrap())
        } else {
            Err(MatchAddError::PastDate)
        }
    } else {
        Err(MatchAddError::IncorrectDateFormat)
    }
}

pub async fn remove_match(conn: &mut PgConnection, match_id: i32) -> bool {
    query!("DELETE FROM matches WHERE id = $1", match_id)
        .execute(conn)
        .await
        .unwrap()
        .rows_affected()
        > 0
}

pub async fn add_set_point_a(conn: &mut PgConnection, match_id: i32) -> bool {
    query!(
        "UPDATE matches SET set_results_a[array_length(set_results_a,1)]=set_results_a[array_length(set_results_a,1)]+1 WHERE id=$1 AND status='IN_PROGRESS' AND set_results_a[array_length(set_results_a,1)]<2147483647",
        match_id
    )
    .execute(conn)
    .await
    .unwrap().rows_affected() > 0
}

pub async fn add_set_point_b(conn: &mut PgConnection, match_id: i32) -> bool {
    query!(
        "UPDATE matches SET set_results_b[array_length(set_results_b,1)]=set_results_b[array_length(set_results_b,1)]+1 WHERE id=$1 AND status='IN_PROGRESS' AND set_results_b[array_length(set_results_b,1)]<2147483647",
        match_id
    )
    .execute(conn)
    .await
    .unwrap().rows_affected() > 0
}

pub async fn remove_set_point_a(conn: &mut PgConnection, match_id: i32) -> bool {
    query!(
        "UPDATE matches SET set_results_a[array_length(set_results_a,1)]=set_results_a[array_length(set_results_a,1)]-1 WHERE id=$1 AND status='IN_PROGRESS' AND set_results_a[array_length(set_results_a,1)]>0",
        match_id
    )
    .execute(conn)
    .await
    .unwrap().rows_affected() > 0
}

pub async fn remove_set_point_b(conn: &mut PgConnection, match_id: i32) -> bool {
    query!(
        "UPDATE matches SET set_results_b[array_length(set_results_b,1)]=set_results_b[array_length(set_results_b,1)]-1 WHERE id=$1 AND status='IN_PROGRESS' AND set_results_b[array_length(set_results_b,1)]>0",
        match_id
    )
    .execute(conn)
    .await
    .unwrap().rows_affected() > 0
}

pub async fn end_set(conn: &mut PgConnection, match_id: i32) -> bool {
    let result=query!(r#"SELECT status "status: MatchStatus", result,set_results_a[array_length(set_results_a,1)] "set_points_a!",set_results_b[array_length(set_results_b,1)] "set_points_b!" FROM matches WHERE id=$1"#,match_id).fetch_one(conn.as_mut()).await.unwrap();
    if result.status == MatchStatus::Finished {
        return false;
    }
    if result.status == MatchStatus::Planned {
        query!(
            "UPDATE matches SET status='IN_PROGRESS',match_start=now(),set_start=now() WHERE id=$1",
            match_id
        )
        .execute(conn.as_mut())
        .await
        .unwrap();
        return true;
    }
    if result.result[0] == 2 && result.result[1] == 2 {
        if (result.set_points_a < 15 && result.set_points_b < 15)
            || result.set_points_a.abs_diff(result.set_points_b) < 2
        {
            return false;
        }
    } else {
        if (result.set_points_a < 25 && result.set_points_b < 25)
            || result.set_points_a.abs_diff(result.set_points_b) < 2
        {
            return false;
        }
    }
    let result_index = if result.set_points_a > result.set_points_b {
        0
    } else {
        1
    };
    if result.result[result_index] >= 2 {
        query!(
            "UPDATE matches SET swapped=FALSE, status='FINISHED' WHERE id=$1",
            match_id
        )
        .execute(conn.as_mut())
        .await
        .unwrap();
    } else {
        query!(
                "UPDATE matches SET set_results_a[array_length(set_results_a,1)+1]=0,set_results_b[array_length(set_results_b,1)+1]=0,set_start=now() WHERE id=$1",
                match_id
            )
            .execute( conn.as_mut())
            .await
            .unwrap();
    }
    QueryBuilder::new("UPDATE matches SET result[")
        .push(result_index + 1)
        .push("]=result[")
        .push(result_index + 1)
        .push("]+1 WHERE id=$1")
        .build()
        .bind(match_id)
        .execute(conn.as_mut())
        .await
        .unwrap();
    true
}
