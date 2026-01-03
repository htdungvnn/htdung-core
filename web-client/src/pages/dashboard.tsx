import { requireAuth } from "@/lib/auth";
import { AuthenticatedUser } from "@/lib/types";
import type { GetServerSideProps, InferGetServerSidePropsType } from "next";

type Props = {
  user: AuthenticatedUser;
};

export const getServerSideProps: GetServerSideProps<Props> = async (ctx) => {
  const user = await requireAuth(ctx);

  if (!user) {
    return {
      redirect: {
        destination: "/login",
        permanent: false,
      },
    };
  }

  return {
    props: {
      user,
    },
  };
};

export default function Dashboard(
  props: InferGetServerSidePropsType<typeof getServerSideProps>
) {
  const { user } = props;

  return (
    <>
      <h1>Dashboard</h1>
      <p>User ID: {user.user_id}</p>
      <p>Role: {user.role}</p>

      <form method="post" action="/api/auth/logout">
        <button>Logout</button>
      </form>
    </>
  );
}