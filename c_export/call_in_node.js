var ffi = require('ffi');

var stringtools = ffi.Library('target/debug/libstringtools.dylib', {
    'count_substrings': ['int', ['string', 'string']]
});

console.log(stringtools.count_substrings("banana", "na"));

