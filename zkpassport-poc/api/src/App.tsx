import { useState } from "react";

const API = import.meta.env.VITE_API_URL || "http://localhost:5174/api";

export default function App() {
  const [health, setHealth] = useState<string>("");
  const [requestId, setRequestId] = useState<string>("");
  const [proof, setProof] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  const checkHealth = async () => {
    const res = await fetch(`${API}/health`);
    setHealth(JSON.stringify(await res.json(), null, 2));
  };

  const issue = async () => {
    setLoading(true);
    setProof(null);
    const res = await fetch(`${API}/issue`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ subject: "alice", claim: "demo-access" })
    });
    const data = await res.json();
    setRequestId(data.requestId || "");
    setLoading(false);
  };

  const verify = async () => {
    if (!requestId) return;
    setLoading(true);
    const res = await fetch(`${API}/verify`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ requestId })
    });
    const data = await res.json();
    setProof(data.proof || data);
    setLoading(false);
  };

  return (
    <div style={{ maxWidth: 680, margin: "2rem auto", fontFamily: "system-ui" }}>
      <h1>Soundness vApp PoC</h1>

      <button onClick={checkHealth}>Check SL Health (mock)</button>
      {health && <pre>{health}</pre>}

      <hr />

      <button disabled={loading} onClick={issue}>Issue Request</button>
      {requestId && <p>Request ID: <code>{requestId}</code></p>}

      <button disabled={loading || !requestId} onClick={verify}>Verify</button>
      {proof && (
        <pre>{JSON.stringify(proof, null, 2)}</pre>
      )}
    </div>
  );
}
