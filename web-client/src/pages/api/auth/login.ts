import type { LoginResponse } from "@/lib/auth-grpc.types";
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

  authClient.login(
    { email, password },
    (err: ServiceError | null, response?: LoginResponse) => {
      if (err || !response) {
        return res.status(401).json({ error: "Invalid credentials" });
      }

      res.setHeader(
        "Set-Cookie",
        `token=${response.token}; HttpOnly; Path=/; SameSite=Lax`
      );

      res.status(200).json({ ok: true });
    }
  );
}