import type {
  ValidateTokenResponse,
} from "@/lib/auth-grpc.types";
import { authClient } from "@/lib/grpc";
import type { ServiceError } from "@grpc/grpc-js";
import type { NextApiRequest, NextApiResponse } from "next";

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  const token = req.cookies.token;
  if (!token) {
    return res.status(401).end();
  }

  authClient.validateToken(
    { token },
    (
      err: ServiceError | null,
      response?: ValidateTokenResponse
    ) => {
      if (err || !response || !response.valid) {
        return res.status(401).end();
      }

      res.status(200).json(response);
    }
  );
}