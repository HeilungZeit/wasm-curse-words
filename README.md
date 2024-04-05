## wasm-curse-words
The wasm-curse-words library provides functionality for detecting and replacing profanity in text. It is written in Rust and compiled to WebAssembly for use in JavaScript.

## Usage

```const WasmCurse = require('wasm-curse-words');

// Create a new instance of WasmCurse
const curse = new WasmCurse('\*', ['en', 'de', 'ru']);

// Check text for profanity
const hasCurseWords = curse.checkCurseWords('Some text to check');
console.log(hasCurseWords); // Outputs true or false

// Define the language of the text
const language = curse.defineLanguage('Some text to check');
console.log(language); // Outputs the language code (e.g., 'en')

// Replace profanity in text
const cleanedText = curse.replaceCurseWords('Some text to check');
console.log(cleanedText); // Outputs the text with profanity replaced
```
## API
Creates a new instance of WasmCurse.
```new WasmCurse(replaceChar, languagesToCheck)```
replaceChar - The character to replace profanity with.
languagesToCheck - An array of language codes to check for profanity.

Checks the text for profanity. Returns true if the text contains profanity, and false otherwise.
```checkCurseWords(text)```

Defines the language of the text. Returns the language code.
```defineLanguage(text)```

Replaces profanity in the text with the character specified when creating the WasmCurse instance. Returns the processed text.
```replaceCurseWords(text)```
