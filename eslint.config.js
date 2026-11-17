import js from "@eslint/js";
import prettier from "eslint-config-prettier";
import betterTailwindcss from "eslint-plugin-better-tailwindcss";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import tseslint from "typescript-eslint";

export default tseslint.config(
	js.configs.recommended,
	...tseslint.configs.recommended,
	...svelte.configs["flat/recommended"],
	...svelte.configs["flat/prettier"],
	prettier,
	{
		languageOptions: {
			globals: { ...globals.browser, ...globals.node },
		},
	},
	{
		files: ["**/*.svelte", "**/*.svelte.ts"],
		languageOptions: {
			parserOptions: {
				parser: tseslint.parser,
				extraFileExtensions: [".svelte"],
			},
		},
	},
	{
		files: ["**/*.svelte", "**/*.{js,ts}"],
		plugins: {
			"better-tailwindcss": betterTailwindcss,
		},
		settings: {
			"better-tailwindcss": {
				entryPoint: "./src/app.css",
			},
		},
		rules: {
			"better-tailwindcss/enforce-canonical-classes": "error",
		},
	},
	{
		ignores: ["build/", ".svelte-kit/", "package/", "src-tauri/", "static/"],
	},
);
