export default function MovieList({ movies, selectedMovie, chooseMovie }) {
  return (
    <aside className="panel">
      <h2>Movies</h2>
      <div className="mt-4 space-y-3">
        {movies.map((movie) => (
          <button
            key={movie.id}
            onClick={() => chooseMovie(movie)}
            className={`movie-row ${selectedMovie.id === movie.id ? "selected" : ""}`}
            type="button"
          >
            <span className={`poster bg-gradient-to-br ${movie.posterTone || "from-zinc-800 to-zinc-500"}`} />
            <span className="min-w-0 text-left">
              <strong>{movie.title}</strong>
              <small>{movie.genre} · {movie.rating}</small>
            </span>
          </button>
        ))}
      </div>
    </aside>
  );
}