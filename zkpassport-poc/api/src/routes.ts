import { Router } from "express";
import { slClient } from "./slClient";

const r = Router();

r.get("/health", async (_req, res) => {
  const status = await slClient.health();
  res.json(status);
});

r.post("/issue", async (req, res) => {
  const { subject, claim } = req.body || {};
  if (!subject || !claim) return res.status(400).json({ error: "subject & claim required" });
  const result = await slClient.issue({ subject, claim });
  res.json(result);
});

r.post("/verify", async (req, res) => {
  const { requestId } = req.body || {};
  if (!requestId) return res.status(400).json({ error: "requestId required" });
  const result = await slClient.verify(requestId);
  res.json(result);
});

export default r;
