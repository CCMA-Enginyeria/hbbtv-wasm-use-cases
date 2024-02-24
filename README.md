In order to illustrate how we would use WASM on HbbTv, we've make the following proof of concept

# POC

## Background

Some years ago we wrote a pure javascript EBU-TT-D parser and renderer, actually it's what we are using now in our hbbtv app (among other apps). [here is the code](https://github.com/CCMA-Enginyeria/ebu-tt-d-parser)

Even, that parser it's quite optimized (ie. preprocesing upfront and using memoize techniques), we've found that some devices wasn't able to process and render subtitle sincronized with video, and we made the decision to disable that feature for lower-performance devices.

## POC

We have translated the JS code to RUST and compiled to WASM. And we have published to websites

Wasm version: https://ccma-labs-generic.s3.eu-west-1.amazonaws.com/hbbtv-wasm-use-cases/subtitles/index.html

Pure JS version: https://ccma-labs-generic.s3.eu-west-1.amazonaws.com/hbbtv-wasm-use-cases/subtitles/index_js.html

During the execution we save performance metrics so we can analyze them later (window.debugTimers).

You could find the source code here: https://github.com/CCMA-Enginyeria/hbbtv-wasm-use-cases

If you want to fork the repo or test on your devices it would be very welcome, and if you need help for that do not hesitate to ask me.

## Results

We're still gathering all the information, as soon as we have a report we'll share them.

...

# Our opinion

Since we're building media application, where we usualy do a lot of tasks synchronized with video, the ability of using WASM will let us give a more rich experiences and reach more devices that would be excluded due to poor performance.