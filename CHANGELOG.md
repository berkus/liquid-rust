<a name="0.14.3"></a>
## 0.14.3 (2018-04-10)


#### Bug Fixes

*   Reduce deps for users ([41e9b01a](https://github.com/cobalt-org/liquid-rust/commit/41e9b01a6b2925562b2ef073a8a420c64f08e570))
* **error:**  Make API consumable by failure ([54be3400](https://github.com/cobalt-org/liquid-rust/commit/54be3400dcebe4944196a36be9c99a5187a6f550))



<a name="0.14.2"></a>
## 0.14.2 (2018-03-16)


#### Features

* **if:**  Bare if is an existence check ([7ab091ca](https://github.com/cobalt-org/liquid-rust/commit/7ab091cadce48d4cb066b3c494fd26f34f0d9625))



<a name="0.14.1"></a>
## 0.14.1 (2018-01-24)


#### Features

* **API:**
  *  Support &String->Scalar ([b87c983c](https://github.com/cobalt-org/liquid-rust/commit/b87c983c1b5fc9061c1d86424b135119d82fe737))
  *  Re-export datetime ([1ca16f5a](https://github.com/cobalt-org/liquid-rust/commit/1ca16f5a90769e427f45e743fdbfd47629e1d178))



<a name="0.14.0"></a>
## 0.14.0 (2018-01-22)


#### Features

API
* **Value:**  Control key order ([7ff0fcd0](https://github.com/cobalt-org/liquid-rust/commit/7ff0fcd04d2570aea5338e03128de62e494bee62), closes [#159](https://github.com/cobalt-org/liquid-rust/issues/159))

Users
* **errors:**
  *  Provide context on compile errors ([c17df1f3](https://github.com/cobalt-org/liquid-rust/commit/c17df1f30b2eec8c0ded04919c73c9d5a1d63377))
  *  Report context during render ([73e26cf7](https://github.com/cobalt-org/liquid-rust/commit/73e26cf786a5883a5fe98d66678134392f107cda), closes [#105](https://github.com/cobalt-org/liquid-rust/issues/105))
* **filters:**  Implement basic `compact` support ([c0eadd5c](https://github.com/cobalt-org/liquid-rust/commit/c0eadd5c384e6d5745036ed344116d137043c154))

#### Breaking Changes

API
*   Reduce string cloning ([3d93928b](https://github.com/cobalt-org/liquid-rust/commit/3d93928b2a9ac378dbb3ca8bd097b1ed7112620f)

Users
* **value:**  Improve value coercion practices ([ebb4f40e](https://github.com/cobalt-org/liquid-rust/commit/ebb4f40e315c280825e74cad60b4cd91bbe06ea0), closes [#99](https://github.com/cobalt-org/liquid-rust/issues/99)

#### Performance

*   Reduce string cloning ([3d93928b](https://github.com/cobalt-org/liquid-rust/commit/3d93928b2a9ac378dbb3ca8bd097b1ed7112620f)

#### Bug Fixes

API
*   Remove warning when no-default-features ([8c43de87](https://github.com/cobalt-org/liquid-rust/commit/8c43de871b437d14ac8da14d283bc906c6dea9f2))

Users
* **filters:**  date_in_tz can't parse cobalt date ([1dae5276](https://github.com/cobalt-org/liquid-rust/commit/1dae52767680c7a2b628f631078a97d1ef37ca37))
* **value:**  Improve value coercion practices ([ebb4f40e](https://github.com/cobalt-org/liquid-rust/commit/ebb4f40e315c280825e74cad60b4cd91bbe06ea0), closes [#99](https://github.com/cobalt-org/liquid-rust/issues/99)



<a name="0.13.7"></a>
## 0.13.7 (2018-01-10)


#### Features

*   Implement `contains` operator ([a0d27205](https://github.com/cobalt-org/liquid-rust/commit/a0d2720570d13d489d7d929452c41334a9d019eb), closes [#155](https://github.com/cobalt-org/liquid-rust/issues/155))



<a name="0.13.6"></a>
## 0.13.6 (2017-12-29)


#### Features

* **filters:**  date can parse YYYY-MM-DD HH:MM:SS TTTT ([59ab76dc](https://github.com/cobalt-org/liquid-rust/commit/59ab76dcd343a6d9d0fff497e6ba2ff1140b3f2a))



<a name="0.13.5"></a>
## 0.13.5 (2017-12-27)

* Update dependencies

<a name="0.13.4"></a>
## 0.13.4 (2017-12-27)


#### Bug Fixes

* **parse:**  Error on empty expressions ([5cffe44a](https://github.com/cobalt-org/liquid-rust/commit/5cffe44a5fb3821dab8a41b8662596421f387659), closes [#139](https://github.com/cobalt-org/liquid-rust/issues/139))
* **raw:**  Stop swapping the text's order ([bd45c14b](https://github.com/cobalt-org/liquid-rust/commit/bd45c14b58e1b22e156b42f3c5629e3a0692e7d4), closes [#79](https://github.com/cobalt-org/liquid-rust/issues/79))



<a name="0.13.3"></a>
## 0.13.3 (2017-12-18)


#### Bug Fixes

* **for:**  Re-enable support for object.access ([cc9998b5](https://github.com/cobalt-org/liquid-rust/commit/cc9998b55a225941fc5d402f414c32abf64c4500))



<a name="0.13.2"></a>
## 0.13.2 (2017-12-18)


#### Features

* **api:**  Add missing traits ([e0f82705](https://github.com/cobalt-org/liquid-rust/commit/e0f82705e25e7ff40d246749e7d8b0da04637813))

#### Bug Fixes

* **nil:**  Equality logic missed a case ([111d10a6](https://github.com/cobalt-org/liquid-rust/commit/111d10a695eaf8d906c77569aac627042d52f8eb))



<a name="0.13.1"></a>
## 0.13.1 (2017-12-17)

Minor docs change.


<a name="0.13.0"></a>
## 0.13.0 (2017-12-17)


#### Features

* **api:**  Make Renderable debuggable ([802b0af0](https://github.com/cobalt-org/liquid-rust/commit/802b0af0045874565d68a4c4f3b957ddef1b44bd))

#### Bug Fixes

* **dbg:**  Remove debug code ([7bf2a3d4](https://github.com/cobalt-org/liquid-rust/commit/7bf2a3d4754252a0c67c7c514e1dca542e565e4c))
* **for:**  Remove non-standard for_loop variable ([0d9515fe](https://github.com/cobalt-org/liquid-rust/commit/0d9515fe1a8c89e9604beb1a69370256d0f23f08), breaks [#](https://github.com/cobalt-org/liquid-rust/issues/))

#### Breaking Changes

* **for:**  Remove non-standard for_loop variable ([0d9515fe](https://github.com/cobalt-org/liquid-rust/commit/0d9515fe1a8c89e9604beb1a69370256d0f23f08), breaks [#](https://github.com/cobalt-org/liquid-rust/issues/))



<a name="0.12.0"></a>
## 0.12.0 (2017-11-29)


#### Features

*   Make LiquidOptions cloneable ([838e5261](https://github.com/cobalt-org/liquid-rust/commit/838e5261b6654aab2a93cb5ff2220f75e2d554df))
  *   Make TemplateRepository cloneable ([94f337ae](https://github.com/cobalt-org/liquid-rust/commit/94f337aee53cdd126001b32427b415b20d70d25a))
  *   Make ParseBlock cloneable ([472fb638](https://github.com/cobalt-org/liquid-rust/commit/472fb638e79ab1126979aecb258990d4b93f2935))
  *   Make ParseTag cloneable ([ec59839d](https://github.com/cobalt-org/liquid-rust/commit/ec59839d9d1deff52bb663d0310d5efbca5acace))

#### Breaking Change

*   Make TemplateRepository cloneable ([94f337ae](https://github.com/cobalt-org/liquid-rust/commit/94f337aee53cdd126001b32427b415b20d70d25a))
*   Make ParseBlock cloneable ([472fb638](https://github.com/cobalt-org/liquid-rust/commit/472fb638e79ab1126979aecb258990d4b93f2935))
*   Make ParseTag cloneable ([ec59839d](https://github.com/cobalt-org/liquid-rust/commit/ec59839d9d1deff52bb663d0310d5efbca5acace))


<a name="0.11.0"></a>
## 0.11.0 (2017-11-08)


#### Features

* **syntax:** Add `arr[0]` and `obj["name"]` indexing (PR #141, fixes #127)
* **value:**  Add nil value to support foreign data (PR #140, [89f6660d](https://github.com/cobalt-org/liquid-rust/commit/89f6660d61ee3a59d3e29e7ad8fe6b31791b1d6f))

#### Breaking Change

* **value:**  Add nil value to support foreign data (PR #140, [89f6660d](https://github.com/cobalt-org/liquid-rust/commit/89f6660d61ee3a59d3e29e7ad8fe6b31791b1d6f))
  * Technically will break anyone matching on `liquid::Value`.

<a name="0.10.1"></a>
## 0.10.1 (2017-09-24)


#### Features

*   Turn `serde` into a default feature. ([6be99f1d](https://github.com/cobalt-org/liquid-rust/commit/6be99f1da4c066dc08eafd6918f604409f93d43d), closes [#128](https://github.com/cobalt-org/liquid-rust/issues/128))

### Bug Fixes
* Stop recompiling everytime due to Skeptic.


<a name="v0.10.0"></a>
## v0.10.0 (2017-05-27)


#### Features

* **filters:**
  *  Add sort_natural ([ef14f871](https://github.com/cobalt-org/liquid-rust/commit/ef14f87151d73e6079450ec46ebd9da805966aa7))
  *  Implement a dummy `compact` ([44d4d061](https://github.com/cobalt-org/liquid-rust/commit/44d4d0619754fbce519a8d51743651d4cee8e00d))
  *  map filter ([52dc03c0](https://github.com/cobalt-org/liquid-rust/commit/52dc03c06a25a037cc65da3f39f46711be62d76c))
  *  Add concat filter ([36d0d2c1](https://github.com/cobalt-org/liquid-rust/commit/36d0d2c1c4250fa16a3a16af2754ba14f6adb62d))
  *  `round` accepts a precision param ([ef691f13](https://github.com/cobalt-org/liquid-rust/commit/ef691f137d6327df7479abd68ae165f282da2aff))
* **Value:**
  *  Add serde support ([8ae7f5a1](https://github.com/cobalt-org/liquid-rust/commit/8ae7f5a1da00434a6c4d7297938164452d943f09) and [839f44b3](https://github.com/cobalt-org/liquid-rust/commit/839f44b3bdce926c8520d77e9a9e35b60d8e522a), closes [#113](https://github.com/cobalt-org/liquid-rust/issues/113))
  *  Add convenience functions ([4b73b3c2](https://github.com/cobalt-org/liquid-rust/commit/4b73b3c2ebb2a48c05052adff8a104187d58943f))
  *  Publicly expose Object and Array ([280c6d99](https://github.com/cobalt-org/liquid-rust/commit/280c6d9956347f7903e719cb55ee14da46ce1465))
* **debug:**  Adding CLI for testing liquid ([171cbfe0](https://github.com/cobalt-org/liquid-rust/commit/171cbfe0ba297c496dbb738ba136b8d6cbce9eb7) and [9d4b4088](https://github.com/cobalt-org/liquid-rust/commit/9d4b408881292cb57c858d144b91a3f626e53f05))
* **performance:**  Add benchmarks ([0e90972d](https://github.com/cobalt-org/liquid-rust/commit/0e90972d620c02f6e587076e093c330287de070b))

#### Bug Fixes

* **filters:**
  *  Align behavior with shopify/liquid ([ebd7ebc6](https://github.com/cobalt-org/liquid-rust/commit/ebd7ebc696b6176e6a8f24b3efb58f5683d1c341))
  *  Moved `pluralize` to `extra-filters` ([17d57c09](https://github.com/cobalt-org/liquid-rust/commit/17d57c093fc8771531c13b6f587b44b2b25d2b03))



