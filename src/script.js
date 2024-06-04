let refresh_timer = null
let clipboard_timer = null;
let current_clipboard = null;
let show_status = [true, true, true];
const button_ids = ['#toggle_planned', '#toggle_in_progress', '#toggle_finished'];
const match_status = ['Planned', 'InProgress', 'Finished'];

function close_error() {
  document.querySelector('#error').innerHTML = '';
}

function cmp(a, b) {
  a = a.children[2].innerHTML;
  b = b.children[2].innerHTML;
  if (a > b) {
    return -1;
  }
  if (a < b) {
    return 1;
  }
  return 0;
}

function check_empty() {
  let match_list = document.querySelector('#match_list');
  let matches = Array.from(match_list.children).filter(e => !e.classList.contains('hidden'));
  document.querySelector('#no_matches').classList.toggle('hidden', matches.length != 0);
  match_list.classList.toggle('hidden', matches.length == 0);
}

function matches_load() {
  for (let i = 0; i < 3; ++i) {
    let button = document.querySelector(button_ids[i]);
    button.classList.toggle('bg-sky-500', show_status[i]);
    button.classList.toggle('hover:bg-sky-400', show_status[i]);
    button.classList.toggle('active:bg-sky-300', show_status[i]);
    button.classList.toggle('bg-zinc-700', !show_status[i]);
    button.classList.toggle('hover:bg-zinc-600', !show_status[i]);
    button.classList.toggle('active:bg-zinc-500', !show_status[i]);
    button.addEventListener('click', (event) => {
      show_status[i] ^= true;
      let match_list = document.querySelector('#match_list');
      Array.from(match_list.children).filter(e => e.children[4].innerHTML == match_status[i]).forEach(e => e.classList.toggle('hidden', !show_status[i]));
      event.currentTarget.classList.toggle('bg-sky-500', show_status[i]);
      event.currentTarget.classList.toggle('hover:bg-sky-400', show_status[i]);
      event.currentTarget.classList.toggle('active:bg-sky-300', show_status[i]);
      event.currentTarget.classList.toggle('bg-zinc-700', !show_status[i]);
      event.currentTarget.classList.toggle('hover:bg-zinc-600', !show_status[i]);
      event.currentTarget.classList.toggle('active:bg-zinc-500', !show_status[i]);
      check_empty();
    });
  }
}

function matches_update() {
  let match_list = document.querySelector('#match_list');
  Array.from(match_list.children).sort(cmp).forEach(e => match_list.appendChild(e));
  for (let i = 0; i < 3; ++i) {
    Array.from(match_list.children).filter(e => e.children[4].innerHTML == match_status[i]).forEach(e => e.classList.toggle('hidden', !show_status[i]));
  }
  check_empty();
}

function handle_clipboard(event, text) {
  navigator.clipboard.writeText(text);
  if (current_clipboard) {
    clearTimeout(clipboard_timer);
    reset_clipboard();
  }
  current_clipboard = event.currentTarget.querySelector('.clipboard');
  current_clipboard.innerHTML = '<svg viewBox="0 0 24 24" class="size-full text-green-400"><use xlink:href="#clipboard-check-icon"></svg>';
  clipboard_timer = setTimeout(reset_clipboard, 1000);
  event.stopPropagation();
}

function reset_clipboard() {
  current_clipboard.innerHTML = '<svg viewBox="0 0 24 24" class="size-full"><use xlink:href="#clipboard-icon"></svg>';
  clipboard_timer = null;
  current_clipboard = null;
}

function start_timer() {
  if (!refresh_timer) {
    refresh_timer = setInterval(update_time, 1000);
  }
  update_time();
}

function format_time(ms) {
  let s = Math.floor(ms / 1000);
  let h = Math.floor(s / 3600);
  s = s % 3600;
  let m = Math.floor(s / 60);
  s = s % 60;
  return h.toString().padStart(2, "0") + ":" + m.toString().padStart(2, "0") + ":" + s.toString().padStart(2, "0");
}

function update_time() {
  try {
    document.querySelector('#current_time').innerHTML = format_time(Date.now() % 86400000);
    document.querySelector('#match_time').innerHTML = format_time(new Date(new Date(Date.now()).toISOString().substring(0, 23)) - new Date(document.querySelector('#match_start').innerHTML));
    document.querySelector('#set_time').innerHTML = format_time(new Date(new Date(Date.now()).toISOString().substring(0, 23)) - new Date(document.querySelector('#set_start').innerHTML));
  } catch (e) {
    if (refresh_timer) {
      clearTimeout(refresh_timer);
      refresh_timer = null;
    }
  }
}
