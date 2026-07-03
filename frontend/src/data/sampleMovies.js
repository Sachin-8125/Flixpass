const sampleMovies = [
  {
    id: "preview-1",
    title: "Neon Horizon",
    genre: "Sci-Fi Thriller",
    rating: "UA 13+",
    durationMinutes: 128,
    synopsis:
      "A pilot and a cryptographer race through a city-wide blackout to stop a synthetic comet from erasing orbital navigation.",
    posterTone: "from-cyan-400 via-slate-900 to-rose-500",
    showtimes: [
      {
        id: "s1",
        startsAt: new Date(Date.now() + 86400000).toISOString(),
        screen: "Screen 2",
        priceCents: 1450,
        totalSeats: 96,
        bookedSeats: [4, 5, 6, 19],
      },
    ],
  },
  {
    id: "preview-2",
    title: "The Last Interval",
    genre: "Drama",
    rating: "PG",
    durationMinutes: 116,
    synopsis:
      "Two rival theatre owners rebuild their friendship during the final weekend of a beloved single-screen cinema.",
    posterTone: "from-amber-300 via-red-900 to-zinc-950",
    showtimes: [
      {
        id: "s2",
        startsAt: new Date(Date.now() + 172800000).toISOString(),
        screen: "Screen 1",
        priceCents: 990,
        totalSeats: 72,
        bookedSeats: [1, 2, 12],
      },
    ],
  },
  {
    id: "preview-3",
    title: "Midnight Courier",
    genre: "Action",
    rating: "A",
    durationMinutes: 141,
    synopsis:
      "A retired stunt rider accepts one last delivery across five boroughs while every camera in the city is looking for him.",
    posterTone: "from-lime-300 via-neutral-900 to-indigo-600",
    showtimes: [
      {
        id: "s3",
        startsAt: new Date(Date.now() + 259200000).toISOString(),
        screen: "IMAX",
        priceCents: 1850,
        totalSeats: 120,
        bookedSeats: [8, 9, 10, 33, 34],
      },
    ],
  },
];

export default sampleMovies;