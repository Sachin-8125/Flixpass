import { useEffect, useState } from "react";
import { API_URL, authHeaders } from "./lib/api.js";
import sampleMovies from "./data/sampleMovies.js";
import AppHeader from "./components/AppHeader.jsx";
import HeroBanner from "./components/HeroBanner.jsx";
import AuthPanel from "./components/AuthPanel.jsx";
import MovieList from "./components/MovieList.jsx";
import BookingPanel from "./components/BookingPanel.jsx";
import Sidebar from "./components/Sidebar.jsx";

export default function App() {
  const [token, setToken] = useState(localStorage.getItem("token") || "");
  const [me, setMe] = useState(JSON.parse(localStorage.getItem("me") || "null"));
  const [movies, setMovies] = useState(sampleMovies);
  const [selectedMovie, setSelectedMovie] = useState(sampleMovies[0]);
  const [selectedShowtime, setSelectedShowtime] = useState(sampleMovies[0].showtimes[0]);
  const [selectedSeats, setSelectedSeats] = useState([]);
  const [bookings, setBookings] = useState([]);
  const [authMode, setAuthMode] = useState("login");
  const [notice, setNotice] = useState("Using preview data until the Rust API is running.");

  useEffect(() => {
    fetch(`${API_URL}/api/movies`)
      .then((res) => (res.ok ? res.json() : Promise.reject()))
      .then((data) => {
        if (data.length) {
          setMovies(data);
          setSelectedMovie(data[0]);
          setSelectedShowtime(data[0].showtimes?.[0]);
          setNotice("");
        }
      })
      .catch(() => {});
  }, []);

  useEffect(() => {
    if (!token) return;
    fetch(`${API_URL}/api/bookings/me`, { headers: authHeaders(token) })
      .then((res) => (res.ok ? res.json() : []))
      .then(setBookings)
      .catch(() => {});
  }, [token]);

  const bookedSeats = selectedShowtime?.bookedSeats ?? [];
  const total = selectedSeats.length * (selectedShowtime?.priceCents ?? 0);
  const isAdmin = me?.role === "ADMIN";

  function chooseMovie(movie) {
    setSelectedMovie(movie);
    setSelectedShowtime(movie.showtimes?.[0]);
    setSelectedSeats([]);
  }

  async function submitAuth(event) {
    event.preventDefault();
    const form = new FormData(event.currentTarget);
    const payload = Object.fromEntries(form.entries());
    const res = await fetch(`${API_URL}/api/auth/${authMode}`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });
    const data = await res.json();
    if (!res.ok) {
      setNotice(data.error || "Authentication failed.");
      return;
    }
    setToken(data.token);
    setMe(data.user);
    localStorage.setItem("token", data.token);
    localStorage.setItem("me", JSON.stringify(data.user));
    setNotice(`Signed in as ${data.user.name}.`);
  }

  async function bookSeats() {
    if (!token) {
      setNotice("Sign in before booking seats.");
      return;
    }
    if (!selectedShowtime || selectedSeats.length === 0) return;
    const res = await fetch(`${API_URL}/api/bookings`, {
      method: "POST",
      headers: { "Content-Type": "application/json", ...authHeaders(token) },
      body: JSON.stringify({ showtimeId: selectedShowtime.id, seats: selectedSeats }),
    });
    const data = await res.json();
    if (!res.ok) {
      setNotice(data.error || "Could not create booking.");
      return;
    }
    setBookings((current) => [data, ...current]);
    setSelectedShowtime({ ...selectedShowtime, bookedSeats: [...bookedSeats, ...selectedSeats] });
    setSelectedSeats([]);
    setNotice("Booking confirmed.");
  }

  function logout() {
    setToken("");
    setMe(null);
    localStorage.clear();
  }

  return (
    <main className="min-h-screen bg-stone-100 text-zinc-950">
      <AppHeader me={me} logout={logout} />
      <section className="mx-auto grid max-w-7xl gap-6 px-5 py-6 lg:grid-cols-[1.2fr_.8fr]">
        <HeroBanner movie={selectedMovie} />
        <AuthPanel me={me} mode={authMode} setMode={setAuthMode} submitAuth={submitAuth} notice={notice} />
      </section>

      <section className="mx-auto grid max-w-7xl gap-6 px-5 pb-8 lg:grid-cols-[330px_1fr_330px]">
        <MovieList movies={movies} selectedMovie={selectedMovie} chooseMovie={chooseMovie} />
        <BookingPanel
          movie={selectedMovie}
          showtime={selectedShowtime}
          setShowtime={(showtime) => {
            setSelectedShowtime(showtime);
            setSelectedSeats([]);
          }}
          selectedSeats={selectedSeats}
          setSelectedSeats={setSelectedSeats}
          bookedSeats={bookedSeats}
          total={total}
          bookSeats={bookSeats}
        />
        <Sidebar
          bookings={bookings}
          isAdmin={isAdmin}
          token={token}
          refreshMovies={() => location.reload()}
        />
      </section>
    </main>
  );
}