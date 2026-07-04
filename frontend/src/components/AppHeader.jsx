import { Clapperboard, Lock, LogOut } from "lucide-react";

export default function AppHeader({ me, logout }) {
  return (
    <header className="border-b border-zinc-200 bg-white/90 backdrop-blur">
      <div className="mx-auto flex max-w-7xl items-center justify-between px-5 py-4">
        <div className="flex items-center gap-3">
          <div className="grid size-11 place-items-center rounded-md bg-zinc-950 text-white">
            <Clapperboard size={22} />
          </div>
          <div>
            <p className="text-xl font-bold leading-tight">Flixpass</p>
            <p className="text-sm text-zinc-500">Secure movie ticket booking</p>
          </div>
        </div>
        <div className="flex items-center gap-3">
          {me ? <span className="hidden text-sm text-zinc-600 sm:inline">{me.name} · {me.role}</span> : null}
          {me ? (
            <button className="icon-btn" onClick={logout} title="Logout">
              <LogOut size={18} />
            </button>
          ) : (
            <span className="badge">
              <Lock size={14} /> JWT secured
            </span>
          )}
        </div>
      </div>
    </header>
  );
}