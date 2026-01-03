import { GetServerSidePropsContext } from "next";
import { AuthenticatedUser } from "./types";

export async function requireAuth(
  ctx: GetServerSidePropsContext
): Promise<AuthenticatedUser | null> {
  const res = await fetch("http://localhost:3000/api/auth/validate", {
    headers: {
      cookie: ctx.req.headers.cookie || "",
    },
  });

  if (!res.ok) return null;
  return res.json() as Promise<AuthenticatedUser>;
}