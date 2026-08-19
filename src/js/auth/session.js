const KEY = 'natra.authenticated';

export function isAuthenticated() {
  return sessionStorage.getItem(KEY) === '1';
}

export function startSession() {
  sessionStorage.setItem(KEY, '1');
}

export function endSession() {
  sessionStorage.removeItem(KEY);
}
