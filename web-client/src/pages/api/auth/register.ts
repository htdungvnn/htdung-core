import { authClient } from "@/lib/grpc";
import type { ServiceError } from "@grpc/grpc-js";
import type { NextApiRequest, NextApiResponse } from "next";

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  const { email, password } = req.body as {
    email: string;
    password: string;
  };

  authClient.register(
    { email, password },
    (err: ServiceError | null) => {
      if (err) {
        return res.status(400).json({ error: "Register failed" });
      }

      res.status(200).json({ ok: true });
    }
  );
}