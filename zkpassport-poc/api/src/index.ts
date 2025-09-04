import express from "express";
import cors from "cors";
import routes from "./routes";

const app = express();
app.use(cors());
app.use(express.json());
app.use("/api", routes);

const PORT = process.env.PORT || 5174;
app.listen(PORT, () => console.log(`[API] listening on :${PORT}`));
