import { LayoutDashboard, Plus } from "lucide-react";
import { API_URL, authHeaders } from "../lib/api.js";

export default function Sidebar({ bookings, isAdmin, token, refreshMovies }) {
  async function createDemoMovie() {
    const res = await fetch(`${API_URL}/api/admin/movies/demo`, {
      method: "POST",
      headers: authHeaders(token),
    });
    if (res.ok) refreshMovies();
  }

  return (
    <aside className="space-y-6">
      <div className="panel">
        <h2>My bookings</h2>
        <div className="mt-4 space-y-3">
          {bookings.length ? (
            bookings.map((booking) => (
              <div className="booking" key={booking.id}>
                <strong>{booking.movieTitle}</strong>
                <span>Seats {booking.seats.join(", ")}</span>
              </div>
            ))
          ) : (
            <p className="text-sm text-zinc-500">Bookings appear here after sign in.</p>
          )}
        </div>
      </div>

      <div className="panel">
        <div className="flex items-center gap-2">
          <LayoutDashboard size={20} />
          <h2>Admin</h2>
        </div>
        <p className="mt-3 text-sm text-zinc-600">Only users with the ADMIN role can create movie inventory.</p>
        <button className="secondary mt-4 w-full" disabled={!isAdmin} onClick={createDemoMovie} type="button">
          <Plus size={18} /> Seed demo movie
        </button>
      </div>
    </aside>
  );
}