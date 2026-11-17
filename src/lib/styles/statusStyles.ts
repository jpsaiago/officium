import type { ApplicationStatus } from "../types/application";

export const STATUS_STYLES: Record<
	ApplicationStatus,
	{ badge: string; dot: string; border: string }
> = {
	saved: {
		badge:
			"bg-slate-200 text-slate-700 dark:bg-slate-700/60 dark:text-slate-300",
		dot: "bg-slate-400",
		border:
			"border-slate-300/40 hover:border-slate-400 dark:border-slate-500/40 dark:hover:border-slate-400",
	},
	applied: {
		badge: "bg-blue-100 text-blue-700 dark:bg-blue-900/50 dark:text-blue-300",
		dot: "bg-blue-500",
		border:
			"border-blue-300/40 hover:border-blue-400 dark:border-blue-600/40 dark:hover:border-blue-500",
	},
	interviewing: {
		badge:
			"bg-yellow-100 text-yellow-700 dark:bg-yellow-900/50 dark:text-yellow-300",
		dot: "bg-yellow-500",
		border:
			"border-yellow-300/40 hover:border-yellow-400 dark:border-yellow-600/40 dark:hover:border-yellow-500",
	},
	offer: {
		badge:
			"bg-emerald-100 text-emerald-700 dark:bg-emerald-900/50 dark:text-emerald-300",
		dot: "bg-emerald-500",
		border:
			"border-emerald-300/40 hover:border-emerald-400 dark:border-emerald-600/40 dark:hover:border-emerald-500",
	},
	rejected: {
		badge: "bg-red-100 text-red-700 dark:bg-red-900/50 dark:text-red-300",
		dot: "bg-red-500",
		border:
			"border-red-300/40 hover:border-red-400 dark:border-red-600/40 dark:hover:border-red-500",
	},
	withdrawn: {
		badge: "bg-zinc-200 text-zinc-600 dark:bg-zinc-700/60 dark:text-zinc-400",
		dot: "bg-zinc-400",
		border:
			"border-zinc-300/40 hover:border-zinc-400 dark:border-zinc-500/40 dark:hover:border-zinc-400",
	},
};
