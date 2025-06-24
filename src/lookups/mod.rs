/// This code was taken from [Vladislav Supalov's](https://github.com/vsupalov)
/// [pokereval-rs](https://github.com/vsupalov/pokereval-rs) library, which in
/// turn was based on Cactus Kev's (aka [Kevin Suffecool](https://suffe.cool/))
/// [Poker Hand Evaluator](https://suffe.cool/poker/evaluator.html) code in C.
///
/// ```txt
/// Copyright (c) 2015 Vladislav Supalov
///
/// Permission is hereby granted, free of charge, to any person obtaining a copy
/// of this software and associated documentation files (the "Software"), to deal
/// in the Software without restriction, including without limitation the rights
/// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
/// copies of the Software, and to permit persons to whom the Software is
/// furnished to do so, subject to the following conditions:
///
/// The above copyright notice and this permission notice shall be included in
/// all copies or substantial portions of the Software.
///
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
/// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
/// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
/// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
/// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
/// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
/// THE SOFTWARE.
/// ```
///
///
/// this is a table lookup for all "flush" hands (e.g.  both
/// flushes and straight-flushes.  entries containing a zero
/// mean that combination is not possible with a five-card
/// flush hand.
pub mod flushes;
pub mod products;
pub mod unique5;
pub mod values;
