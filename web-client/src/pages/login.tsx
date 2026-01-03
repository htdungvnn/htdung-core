export default function Login() {
  async function submit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const f = e.currentTarget;

    await fetch("/api/auth/login", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        email: f.email.value,
        password: f.password.value,
      }),
    });

    window.location.href = "/dashboard";
  }

  return (
    <form onSubmit={submit}>
      <h1>Login</h1>
      <input name="email" placeholder="email" />
      <input name="password" type="password" placeholder="password" />
      <button>Login</button>
    </form>
  );
}