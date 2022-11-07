const fs = require("fs");

const coefficients = [
  1.648721270700128,  1.284025416687742,
  1.133148453066826,  1.064494458917859,
  1.031743407499103,  1.015747708586686,
  1.007843097206488,  1.003913889338348,
  1.001955033591003,  1.000977039492417,
  1.000488400478694,  1.000244170429748,
  1.000122077763384,  1.000061037018933,
  1.000030518043791,  1.0000152589054785,
  1.000007629423635,  1.0000038147045416,
  1.0000019073504518, 1.0000009536747712,
  1.000000476837272,  1.0000002384186075,
  1.0000001192092967, 1.0000000596046466,
  1.0000000298023228
];

let program = [];

program.push({
  "function": "cordic_lut",
  args: ["n"],
  block:[{
    "if": coefficients.map((v, i) => ({
      cond: {binop: "==", argl: "n", argr: i}, "then": [{"return": v}]
    }))
  }]
});

// Dummy
program.push({
  "function":	"floor",
  "args":	["x"],
  "block":	[]
})

// Generated with jsl from cordic.lua
program.push({
  "function":	"ln_cordic",
  "args":	["x"],
  "block":	[{
      "declare":	"result",
      "value":	0
    }, {
      "declare":	"e",
      "value":	Math.E
    }, {
      "while":	{
        "binop":	"<=",
        "argl":	"e",
        "argr":	"x"
      },
      "do":	[{
          "set":	"result",
          "value":	{
            "binop":	"+",
            "argl":	"result",
            "argr":	1
          }
        }, {
          "set":	"x",
          "value":	{
            "binop":	"/",
            "argl":	"x",
            "argr":	"e"
          }
        }]
    }, {
      "while":	{
        "binop":	"<",
        "argl":	"x",
        "argr":	1
      },
      "do":	[{
          "set":	"result",
          "value":	{
            "binop":	"-",
            "argl":	"result",
            "argr":	1
          }
        }, {
          "set":	"x",
          "value":	{
            "binop":	"*",
            "argl":	"x",
            "argr":	"e"
          }
        }]
    }, {
      "declare":	"w",
      "value":	0
    }, {
      "declare":	"power",
      "value":	{
        "binop":	"/",
        "argl":	1,
        "argr":	2
      }
    }, {
      "iterator":	"i",
      "from":	0,
      "to":	24,
      "do":	[{
          "if":	[{
              "cond":	{
                "binop":	"<",
                "argl":	{
                  "call":	"cordic_lut",
                  "args":	["i"]
                },
                "argr":	"x"
              },
              "then":	[{
                  "set":	"result",
                  "value":	{
                    "binop":	"+",
                    "argl":	"result",
                    "argr":	"power"
                  }
                }, {
                  "set":	"x",
                  "value":	{
                    "binop":	"/",
                    "argl":	"x",
                    "argr":	{
                      "call":	"cordic_lut",
                      "args":	["i"]
                    }
                  }
                }]
            }]
        }, {
          "set":	"power",
          "value":	{
            "binop":	"/",
            "argl":	"power",
            "argr":	2
          }
        }]
    }, {
      "set":	"x",
      "value":	{
        "binop":	"-",
        "argl":	"x",
        "argr":	1
      }
    }, {
      "set":	"x",
      "value":	{
        "binop":	"*",
        "argl":	{
          "binop":	"*",
          "argl":	{
            "binop":	"*",
            "argl":	"x",
            "argr":	{
              "binop":	"-",
              "argl":	1,
              "argr":	{
                "binop":	"/",
                "argl":	"x",
                "argr":	2
              }
            }
          },
          "argr":	{
            "binop":	"+",
            "argl":	1,
            "argr":	{
              "binop":	"/",
              "argl":	"x",
              "argr":	3
            }
          }
        },
        "argr":	{
          "binop":	"-",
          "argl":	1,
          "argr":	{
            "binop":	"/",
            "argl":	"x",
            "argr":	4
          }
        }
      }
    }, {
      "return":	{
        "binop":	"+",
        "argl":	"result",
        "argr":	"x"
      }
    }]
});

program.push({
  "function":	"exp_cordic",
  "args":	["x"],
  "block":	[{
      "declare":	"e",
      "value":	Math.E
    }, {
      "declare":	"int_part",
      "value":	{
        "call":	"floor",
        "args":	["x"]
      }
    }, {
      "declare":	"result",
      "value":	1
    }, {
      "while":	{
        "binop":	">",
        "argl":	"int_part",
        "argr":	0
      },
      "do":	[{
          "set":	"int_part",
          "value":	{
            "binop":	"-",
            "argl":	"int_part",
            "argr":	1
          }
        }, {
          "set":	"result",
          "value":	{
            "binop":	"*",
            "argl":	"result",
            "argr":	"e"
          }
        }]
    }, {
      "while":	{
        "binop":	"<",
        "argl":	"int_part",
        "argr":	0
      },
      "do":	[{
          "set":	"int_part",
          "value":	{
            "binop":	"+",
            "argl":	"int_part",
            "argr":	1
          }
        }, {
          "set":	"result",
          "value":	{
            "binop":	"/",
            "argl":	"result",
            "argr":	"e"
          }
        }]
    }, {
      "declare":	"z",
      "value":	{
        "binop":	"-",
        "argl":	"x",
        "argr":	{
          "call":	"floor",
          "args":	["x"]
        }
      }
    }, {
      "declare":	"power",
      "value":	{
        "binop":	"/",
        "argl":	1,
        "argr":	2
      }
    }, {
      "iterator":	"i",
      "from":	0,
      "to":	24,
      "do":	[{
          "if":	[{
              "cond":	{
                "binop":	"<",
                "argl":	"power",
                "argr":	"z"
              },
              "then":	[{
                  "set":	"result",
                  "value":	{
                    "binop":	"*",
                    "argl":	"result",
                    "argr":	{
                      "call":	"cordic_lut",
                      "args":	["i"]
                    }
                  }
                }, {
                  "set":	"z",
                  "value":	{
                    "binop":	"-",
                    "argl":	"z",
                    "argr":	"power"
                  }
                }]
            }]
        }, {
          "set":	"power",
          "value":	{
            "binop":	"/",
            "argl":	"power",
            "argr":	2
          }
        }]
    }, {
      "set":	"result",
      "value":	{
        "binop":	"*",
        "argl":	"result",
        "argr":	{
          "binop":	"+",
          "argl":	1,
          "argr":	{
            "binop":	"*",
            "argl":	"z",
            "argr":	{
              "binop":	"+",
              "argl":	1,
              "argr":	{
                "binop":	"*",
                "argl":	{
                  "binop":	"/",
                  "argl":	"z",
                  "argr":	2
                },
                "argr":	{
                  "binop":	"+",
                  "argl":	1,
                  "argr":	{
                    "binop":	"*",
                    "argl":	{
                      "binop":	"/",
                      "argl":	"z",
                      "argr":	3
                    },
                    "argr":	{
                      "binop":	"+",
                      "argl":	1,
                      "argr":	{
                        "binop":	"/",
                        "argl":	"z",
                        "argr":	4
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }, {
      "return":	"result"
    }]
})

program.push({
  "function":	"pow",
  "args":	["base", "exponent"],
  "block":	[{
      "declare":	"result",
      "value":	1
    }, {
      "while":	{
        "binop":	">=",
        "argl":	"exponent",
        "argr":	1
      },
      "do":	[{
          "set":	"exponent",
          "value":	{
            "binop":	"-",
            "argl":	"exponent",
            "argr":	1
          }
        }, {
          "set":	"result",
          "value":	{
            "binop":	"*",
            "argl":	"result",
            "argr":	"base"
          }
        }]
    }, {
      "while":	{
        "binop":	"<",
        "argl":	"exponent",
        "argr":	0
      },
      "do":	[{
          "set":	"exponent",
          "value":	{
            "binop":	"+",
            "argl":	"exponent",
            "argr":	1
          }
        }, {
          "set":	"result",
          "value":	{
            "binop":	"/",
            "argl":	"result",
            "argr":	"base"
          }
        }]
    }, {
      "if":	[{
          "cond":	{
            "binop":	"~=",
            "argl":	"exponent",
            "argr":	0
          },
          "then":	[{
              "return":	{
                "binop":	"*",
                "argl":	"result",
                "argr":	{
                  "call":	"exp_cordic",
                  "args":	[{
                      "binop":	"*",
                      "argl":	{
                        "call":	"ln_cordic",
                        "args":	["base"]
                      },
                      "argr":	"exponent"
                    }]
                }
              }
            }]
        }]
    }, {
      "return":	"result"
    }]
});

fs.writeFileSync("cordic.lang.json", JSON.stringify(program, null, 1));