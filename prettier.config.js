/** @type {import('prettier').Config} */
export default {
	useTabs: true,
	trailingComma: "all",
	plugins: [
		"prettier-plugin-svelte",
		"prettier-plugin-tailwindcss",
		"@ianvs/prettier-plugin-sort-imports",
	],
	tailwindStylesheet: "./src/app.css",
	tailwindFunctions: ["cva", "cx"],
	overrides: [
		{
			files: "*.svelte",
			options: { parser: "svelte" },
		},
	],
	importOrder: [
		"<TYPES>^(node:)",
		"<TYPES>",
		"<TYPES>^[.]",
		"<BUILTIN_MODULES>",
		"<THIRD_PARTY_MODULES>",
		"^[.]",
	],
	importOrderParserPlugins: ["typescript", "jsx", "decorators-legacy"],
	importOrderTypeScriptVersion: "5.0.0",
	importOrderCaseSensitive: false,
};
