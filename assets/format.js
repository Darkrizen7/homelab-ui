const GB = 1024 ** 3;
const MB = 1024 ** 2;

export function bytes(value) {
  if (value == null) return '—';
  if (value >= GB) return `${(value / GB).toFixed(value >= 10 * GB ? 0 : 1)} Go`;
  if (value >= MB) return `${Math.round(value / MB)} Mo`;
  return `${Math.max(1, Math.round(value / 1024))} Ko`;
}

export function percent(value, digits = 0) {
  return value == null ? '—' : value.toFixed(digits);
}

export function duration(seconds) {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  if (days > 0) return `${days} j ${hours} h`;
  if (hours > 0) return `${hours} h ${minutes} min`;
  return `${minutes} min`;
}

export function clock(timestamp) {
  return new Date(timestamp * 1000).toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' });
}

/// Docker status in French: "Up 2 hours (healthy)" → "En marche · 2 hours · sain".
export function status(container) {
  const health = /\((healthy|unhealthy|health: starting)\)/.exec(container.status)?.[1];
  const healthLabel = { healthy: 'sain', unhealthy: 'en échec', 'health: starting': 'démarrage' }[health];
  const since = container.status.replace(/^(Up|Exited \(\d+\))\s*/, '').replace(/\s*\(.*\)$/, '');
  const state = {
    running: 'En marche',
    exited: 'Arrêté',
    restarting: 'Redémarre',
    paused: 'En pause',
    created: 'Créé',
    dead: 'Mort',
  }[container.state] ?? container.state;
  return [state, since, healthLabel].filter(Boolean).join(' · ');
}

export function el(tag, className, text) {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text != null) node.textContent = text;
  return node;
}
