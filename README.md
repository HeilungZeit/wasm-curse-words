## wasm-curse-words

The `wasm-curse-words` library, written in Rust and compiled to WebAssembly, offers functionality for detecting and replacing profanity within text. It integrates seamlessly with JavaScript environments (tested with [NestJs](https://nestjs.com/)).

## Installation

Use the package manager to install `wasm-curse-words`.

```bash
yarn wasm-curse-words
npm i wasm-curse-words
```

## Usage (with Code Highlighting)

```javascript
const WasmCurse = require('wasm-curse-words');

// Create a WasmCurse instance (notice the variable name)
const curse = new WasmCurse('*', ['en', 'de', 'ru']);

// Check text for profanity
const hasCurseWords = curse.checkCurseWords('Some text to check');
console.log(hasCurseWords); // Outputs true or false

// Define the language of the text (optional)
const language = curse.defineLanguage('Some text to check');
console.log(language); // Outputs the language code (e.g., 'en')

// Replace profanity in text
const cleanedText = curse.replaceCurseWords('Some text to check');
console.log(cleanedText); // Outputs the text with profanity replaced
```

## API

```javascript
new WasmCurse(replaceChar, languagesToCheck);
```

Creates a new instance of WasmCurse.

`replaceChar` - The character to replace profanity with.
`languagesToCheck` - An array of language codes to check for profanity. Valid values: `'ru', 'en', 'de', 'pl'`. If no languages was provided in use will be only english.

`checkCurseWords(text)` - checks the text for profanity. Returns true if the text contains profanity, and false otherwise.

`defineLanguage(text)` - defines the language of the text. Returns the language code.

`replaceCurseWords(text)` - replaces profanity in the text with the character specified when creating the WasmCurse instance. Returns the processed text.

## Change log

`v1.1.0` - added support of Polish language; small optimizations;
`v1.0.1` - added panic to `defineLanguage` method when can't define a lang;
`v1.0.0` - migration to class;
