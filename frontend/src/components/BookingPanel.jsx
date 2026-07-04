import  { useMemo } from "react";
import { CalendarDays, CreditCard, Ticket } from "lucide-react";
import { money } from "../lib/money.js";

export default function BookingPanel({
  movie,
  showtime,
  setShowtime,
  selectedSeats,
  setSelectedSeats,
  bookedSeats,
  total,
  bookSeats,
}) {
  const seats = useMemo(() => Array.from({ length: showtime?.totalSeats ?? 0 }, (_, i) => i + 1), [showtime]);

  return (
    <section className="panel">
      <div className="flex items-center justify-between gap-4">
        <h2>Select seats</h2>
        <span className="badge">
          <Ticket size={14} /> {selectedSeats.length} selected
        </span>
      </div>

      <div className="mt-4 grid gap-3 md:grid-cols-2">
        {(movie.showtimes || []).map((item) => (
          <button
            key={item.id}
            className={`showtime ${showtime?.id === item.id ? "selected" : ""}`}
            onClick={() => setShowtime(item)}
            type="button"
          >
            <CalendarDays size={18} />
            <span className="min-w-0">
              <span className="block">
                {new Date(item.startsAt).toLocaleString([], { dateStyle: "medium", timeStyle: "short" })}
              </span>
              <span className="block text-xs font-bold text-zinc-500">{item.screen}</span>
            </span>
            <strong>{money(item.priceCents)}</strong>
          </button>
        ))}
      </div>

      <div className="mt-4 flex flex-wrap gap-3 text-xs font-bold text-zinc-600">
        <span className="inline-flex items-center gap-2 rounded-full bg-zinc-100 px-3 py-1">
          <span className="size-3 rounded-sm border border-zinc-300 bg-zinc-50" />
          Available
        </span>
        <span className="inline-flex items-center gap-2 rounded-full bg-rose-50 px-3 py-1 text-rose-700">
          <span className="size-3 rounded-sm bg-rose-600" />
          Selected
        </span>
        <span className="inline-flex items-center gap-2 rounded-full bg-zinc-100 px-3 py-1">
          <span className="size-3 rounded-sm bg-zinc-800" />
          Booked
        </span>
      </div>

      <div className="screen">SCREEN</div>

      <div className="seat-grid">
        {seats.map((seat) => {
          const booked = bookedSeats.includes(seat);
          const selected = selectedSeats.includes(seat);
          return (
            <button
              key={seat}
              disabled={booked}
              className={`seat ${booked ? "booked" : ""} ${selected ? "selected" : ""}`}
              onClick={() =>
                setSelectedSeats(selected ? selectedSeats.filter((n) => n !== seat) : [...selectedSeats, seat])
              }
              title={`Seat ${seat}`}
              type="button"
            >
              {seat}
            </button>
          );
        })}
      </div>

      <div className="mt-6 flex flex-wrap items-center justify-between gap-3 border-t border-zinc-200 pt-5">
        <div>
          <p className="text-sm text-zinc-500">Total payable</p>
          <strong className="text-2xl">{money(total)}</strong>
        </div>
        <button className="primary" disabled={!selectedSeats.length} onClick={bookSeats} type="button">
          <CreditCard size={18} /> Confirm booking
        </button>
      </div>
    </section>
  );
}