export type Theme = "light" | "dark";

function getInitialTheme(): Theme {
	if (typeof document !== "undefined") {
		return document.documentElement.classList.contains("dark")
			? "dark"
			: "light";
	}
	return "light";
}

export const themeState = $state({ theme: getInitialTheme() });

export function setTheme(theme: Theme) {
	themeState.theme = theme;
	document.documentElement.classList.toggle("dark", theme === "dark");
	try {
		localStorage.setItem("theme", theme);
	} catch {
		// localStorage unavailable (e.g. private mode) — theme just won't persist
	}
}

export function toggleTheme() {
	setTheme(themeState.theme === "dark" ? "light" : "dark");
}
