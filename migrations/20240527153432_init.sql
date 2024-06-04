CREATE TYPE match_status AS ENUM ('PLANNED', 'IN_PROGRESS', 'FINISHED');

CREATE TABLE matches (
    id SERIAL PRIMARY KEY,
    match_start TIMESTAMP NOT NULL DEFAULT now(),
    set_start TIMESTAMP NOT NULL DEFAULT now(),
    team_a VARCHAR NOT NULL,
    team_b VARCHAR NOT NULL,
    swapped BOOLEAN NOT NULL DEFAULT FALSE,
    result INT[2] NOT NULL DEFAULT '{0,0}',
    set_results_a INT[] NOT NULL DEFAULT '{0}',
    set_results_b INT[] NOT NULL DEFAULT '{0}',
    status match_status NOT NULL DEFAULT 'IN_PROGRESS'
);

INSERT INTO matches(match_start,set_start,team_a,team_b,result,set_results_a,set_results_b,status) VALUES('2024-09-01T12:30','2024-09-01T12:30','Trefl Gdańsk','Skra Bełchatów','{3,0}','{25,25,26}','{20,14,24}','FINISHED'),('2024-09-02T06:21','2024-09-02T06:21','Jastrzębski Węgiel','GKS Katowice','{2,3}','{26,20,25,15,12}','{24,25,23,25,15}','FINISHED'),('2024-09-03T16:00','2024-09-03T16:00','Asseco Resovia Rzeszów','Enea Czarni Radom','{3,1}','{12,25,25,25}','{25,14,17,13}','FINISHED');
