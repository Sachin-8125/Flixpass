import { Film } from "lucide-react";

export default function HeroBanner({ movie }) {
  return (
    <div className="overflow-hidden rounded-lg bg-zinc-950 text-white shadow-xl">
      <div className={`min-h-[370px] bg-gradient-to-br ${movie.posterTone || "from-sky-400 via-zinc-950 to-rose-500"} p-8`}>
        <div className="max-w-2xl">
          <span className="badge-dark">
            <Film size={15} /> Now booking
          </span>
          <h1 className="mt-8 max-w-xl text-5xl font-black leading-[1.02] sm:text-6xl">{movie.title}</h1>
          <p className="mt-5 max-w-xl text-lg text-white/82">{movie.synopsis}</p>
          <div className="mt-7 flex flex-wrap gap-3 text-sm">
            <span className="chip">{movie.genre}</span>
            <span className="chip">{movie.rating}</span>
            <span className="chip">{movie.durationMinutes} min</span>
          </div>
        </div>
      </div>
    </div>
  );
}