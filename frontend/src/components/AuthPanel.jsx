import { ShieldCheck, User } from "lucide-react";

export default function AuthPanel({ me, mode, setMode, submitAuth, notice }) {
  if (me) {
    return (
      <aside className="panel">
        <div className="flex items-center gap-3">
          <ShieldCheck className="text-emerald-600" />
          <h2>Account verified</h2>
        </div>
        <p className="mt-3 text-sm text-zinc-600">
          Your token is stored locally and sent as a bearer token. Admin features are protected by the backend role guard.
        </p>
        {notice ? <p className="notice">{notice}</p> : null}
      </aside>
    );
  }

  return (
    <aside className="panel">
      <div className="mb-4 flex rounded-md bg-zinc-100 p-1">
        {["login", "register"].map((item) => (
          <button
            key={item}
            className={`seg ${mode === item ? "active" : ""}`}
            onClick={() => setMode(item)}
            type="button"
          >
            {item}
          </button>
        ))}
      </div>
      <form onSubmit={submitAuth} className="space-y-3">
        {mode === "register" ? <input name="name" placeholder="Full name" required /> : null}
        <input name="email" type="email" placeholder="Email" required />
        <input name="password" type="password" placeholder="Password" minLength="8" required />
        <button className="primary w-full">
          <User size={18} /> {mode === "login" ? "Sign in" : "Create account"}
        </button>
      </form>
      {notice ? <p className="notice">{notice}</p> : null}
    </aside>
  );
}