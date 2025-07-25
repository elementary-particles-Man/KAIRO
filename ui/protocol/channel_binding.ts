// UI ↔ KAIRO-P Channel Binding Layer
export interface UIPost {
  p_address: string;
  payload: string;
  timestamp: string;
  signature: string;
}

export interface UIReceive {
  from: string;
  message: string;
  verified: boolean;
  timestamp: string;
}

export async function postToDaemon(post: UIPost): Promise<boolean> {
  const res = await fetch(`http://localhost:3030/send`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(post),
  });
  return res.ok;
}
