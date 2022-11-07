const fs = require("fs");
const cp = require("child_process");

let didPrintErr = false;
let totalTests = 0;
let totalPass = 0;
function test(name, cb) {
  totalTests++;
  try {
    cb();
    console.log(`Test ${name} OK`)
    totalPass++;
  } catch(err) {
    console.log(`Test ${name} FAIL`)
    if (!didPrintErr) console.log(err);
    didPrintErr = true;
  }
}

function assertEquals(x, y) {
  if (x != y) throw new Error(`Expected ${y} but found ${x}`);
}
function assertAlmostEquals(x, y, delta) {
  if (Math.abs(x - y) > delta) throw new Error(`Expected ${y} but found ${x}`);
}

function run(code) {
  fs.writeFileSync("./test.jsonlang", JSON.stringify(code));
  cp.execSync("cargo run test.jsonlang test.wat 2> /dev/null");
  cp.execSync("wat2wasm test.wat -o test.wasm 2> /dev/null");
  return JSON.parse(cp.execSync("node runcode.js 2> /dev/null").toString())
}

test("040", () => {
  const c = run([{
    "set": "i",
    "value": 100,
  }, {
    "set": "x",
    "value": 0,
  }, {
    "iterator": "i",
    "from": 0,
    "to": 8,
    "do": [{
      "set": "x",
      "value": {
        "binop": "+",
        "argl": "x",
        "argr": "i",
      },
    }],
  }, {
    "set": "out",
    "value": {
      "binop": "+",
      "argl": "x",
      "argr": "i",
    },
  }]);
  assertEquals(c, 136);
});

test("026", () => {
  const c = run([{
    "set": "x",
    "value": {
      "unop": "-",
      "arg": 10,
    },
  }, {
    "if": [{
      "cond": {
        "binop": ">",
        "argl": "x",
        "argr": 0,
      },
      "then": [{
        "set": "y",
        "value": 44,
      }],
    }],
    "else": [{
      "set": "y",
      "value": 88,
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, 88);
});

test("029", () => {
  const c = run([{
    "set": "x",
    "value": 100,
  }, {
    "if": [{
      "cond": {
        "binop": ">",
        "argl": "x",
        "argr": 0,
      },
      "then": [{
        "set": "y",
        "value": 1,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 20,
        },
      },
      "then": [{
        "set": "y",
        "value": 2,
      }],
    }],
    "else": [{
      "set": "y",
      "value": 3,
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, 1);
});

test("034", () => {
  const c = run([{
    "set": "x",
    "value": {
      "unop": "-",
      "arg": 55,
    },
  }, {
    "if": [{
      "cond": {
        "binop": ">",
        "argl": "x",
        "argr": 0,
      },
      "then": [{
        "set": "y",
        "value": 1,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 100,
        },
      },
      "then": [{
        "set": "y",
        "value": 2,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 50,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 50,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 60,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 60,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 65,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 65,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 75,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 75,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 90,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 90,
        },
      }],
    }],
    "else": [{
      "set": "y",
      "value": 3,
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, -50);
});

test("028", () => {
  const c = run([{
    "set": "x",
    "value": {
      "unop": "-",
      "arg": 50,
    },
  }, {
    "if": [{
      "cond": {
        "binop": ">",
        "argl": "x",
        "argr": 0,
      },
      "then": [{
        "set": "y",
        "value": 1,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 20,
        },
      },
      "then": [{
        "set": "y",
        "value": 2,
      }],
    }],
    "else": [{
      "set": "y",
      "value": 3,
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, 2);
});

test("036", () => {
  const c = run([{
    "set": "x",
    "value": {
      "unop": "-",
      "arg": 100,
    },
  }, {
    "set": "y",
    "value": 0,
  }, {
    "while": {
      "binop": "<",
      "argl": "x",
      "argr": 100,
    },
    "do": [{
      "set": "y",
      "value": {
        "binop": "+",
        "argl": {
          "binop": "+",
          "argl": "y",
          "argr": "x",
        },
        "argr": 1,
      },
    }, {
      "if": [{
        "cond": {
          "binop": "<",
          "argl": "x",
          "argr": 50,
        },
        "then": [{
          "set": "x",
          "value": {
            "binop": "+",
            "argl": "x",
            "argr": 4,
          },
        }],
      }],
    }, {
      "set": "x",
      "value": {
        "binop": "+",
        "argl": "x",
        "argr": 20,
      },
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, -31);
});

test("025", () => {
  const c = run([{
    "set": "x",
    "value": 1,
  }, {
    "if": [{
      "cond": {
        "binop": ">",
        "argl": "x",
        "argr": 0,
      },
      "then": [{
        "set": "y",
        "value": 44,
      }],
    }],
    "else": [{
      "set": "y",
      "value": 88,
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, 44);
});

test("042", () => {
  const c = run([{
    "set": "x",
    "value": {
      "unop": "-",
      "arg": 55,
    },
  }, {
    "set": "y",
    "value": 0,
  }, {
    "set": "z",
    "value": 0,
  }, {
    "while": {
      "binop": "<",
      "argl": "x",
      "argr": 100,
    },
    "do": [{
      "set": "y",
      "value": {
        "binop": "+",
        "argl": {
          "binop": "+",
          "argl": "y",
          "argr": "x",
        },
        "argr": 1,
      },
    }, {
      "set": "x",
      "value": {
        "binop": "+",
        "argl": "x",
        "argr": 10,
      },
    }, {
      "if": [{
        "cond": {
          "binop": ">",
          "argl": "x",
          "argr": 50,
        },
        "then": ["break"],
      }],
    }, {
      "set": "z",
      "value": {
        "binop": "+",
        "argl": "z",
        "argr": 1,
      },
    }],
  }, {
    "set": "out",
    "value": {
      "binop": "+",
      "argl": "y",
      "argr": "z",
    },
  }]);
  assertEquals(c, -34);
});

test("041", () => {
  const c = run([{
    "set": "i",
    "value": 100,
  }, {
    "set": "x",
    "value": 0,
  }, {
    "iterator": "i",
    "from": 0,
    "to": 8,
    "step": 2,
    "do": [{
      "set": "x",
      "value": {
        "binop": "+",
        "argl": "x",
        "argr": "i",
      },
    }],
  }, {
    "set": "out",
    "value": {
      "binop": "+",
      "argl": "x",
      "argr": "i",
    },
  }]);
  assertEquals(c, 120);
});

test("038", () => {
  const c = run([{
    "set": "x",
    "value": 20,
  }, {
    "set": "y",
    "value": 0,
  }, {
    "do": [{
      "set": "x",
      "value": {
        "binop": "+",
        "argl": "x",
        "argr": 1,
      },
    }, {
      "set": "y",
      "value": {
        "binop": "+",
        "argl": "y",
        "argr": 2,
      },
    }],
    "until": {
      "binop": ">",
      "argl": "y",
      "argr": "x",
    },
  }, {
    "set": "out",
    "value": {
      "binop": "*",
      "argl": "y",
      "argr": "x",
    },
  }]);
  assertEquals(c, 1722);
});

test("037", () => {
  const c = run([{
    "set": "x",
    "value": 0,
  }, {
    "set": "y",
    "value": 0,
  }, {
    "while": {
      "binop": "<",
      "argl": "x",
      "argr": 12,
    },
    "do": [{
      "set": "y",
      "value": {
        "binop": "+",
        "argl": "y",
        "argr": "x",
      },
    }, {
      "set": "x",
      "value": {
        "binop": "+",
        "argl": "x",
        "argr": 1,
      },
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, 66);
});

test("035", () => {
  const c = run([{
    "set": "x",
    "value": {
      "unop": "-",
      "arg": 55,
    },
  }, {
    "set": "y",
    "value": 0,
  }, {
    "while": {
      "binop": "<",
      "argl": "x",
      "argr": 100,
    },
    "do": [{
      "set": "y",
      "value": {
        "binop": "+",
        "argl": {
          "binop": "+",
          "argl": "y",
          "argr": "x",
        },
        "argr": 1,
      },
    }, {
      "set": "x",
      "value": {
        "binop": "+",
        "argl": "x",
        "argr": 10,
      },
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, 336);
});

test("039", () => {
  const c = run([{
    "set": "x",
    "value": 20,
  }, {
    "set": "y",
    "value": 0,
  }, {
    "do": [{
      "set": "x",
      "value": {
        "binop": "+",
        "argl": "x",
        "argr": 1,
      },
    }, {
      "set": "y",
      "value": {
        "binop": "+",
        "argl": "y",
        "argr": 2,
      },
    }],
    "until": 1,
  }, {
    "set": "out",
    "value": {
      "binop": "*",
      "argl": "y",
      "argr": "x",
    },
  }]);
  assertEquals(c, 42);
});

test("030", () => {
  const c = run([{
    "set": "x",
    "value": 100,
  }, {
    "if": [{
      "cond": {
        "binop": ">",
        "argl": "x",
        "argr": 0,
      },
      "then": [{
        "set": "y",
        "value": 1,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 100,
        },
      },
      "then": [{
        "set": "y",
        "value": 2,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 50,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 50,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 60,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 60,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 65,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 65,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 75,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 75,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 90,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 90,
        },
      }],
    }],
    "else": [{
      "set": "y",
      "value": 3,
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, 1);
});

test("045", () => {
  const c = run([{
    "set": "i",
    "value": 100,
  }, {
    "set": "x",
    "value": 0,
  }, {
    "iterator": "i",
    "from": 0,
    "to": 8,
    "step": 2,
    "do": [{
      "if": [{
        "cond": {
          "binop": "==",
          "argl": "x",
          "argr": 2,
        },
        "then": ["break"],
      }],
    }, {
      "set": "x",
      "value": {
        "binop": "+",
        "argl": "x",
        "argr": "i",
      },
    }],
  }, {
    "set": "out",
    "value": {
      "binop": "+",
      "argl": "x",
      "argr": "i",
    },
  }]);
  assertEquals(c, 102);
});

test("032", () => {
  const c = run([{
    "set": "x",
    "value": {
      "unop": "-",
      "arg": 80,
    },
  }, {
    "if": [{
      "cond": {
        "binop": ">",
        "argl": "x",
        "argr": 0,
      },
      "then": [{
        "set": "y",
        "value": 1,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 100,
        },
      },
      "then": [{
        "set": "y",
        "value": 2,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 50,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 50,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 60,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 60,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 65,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 65,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 75,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 75,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 90,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 90,
        },
      }],
    }],
    "else": [{
      "set": "y",
      "value": 3,
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, -50);
});

test("027", () => {
  const c = run([{
    "set": "x",
    "value": {
      "unop": "-",
      "arg": 10,
    },
  }, {
    "if": [{
      "cond": {
        "binop": ">",
        "argl": "x",
        "argr": 0,
      },
      "then": [{
        "set": "y",
        "value": 1,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 20,
        },
      },
      "then": [{
        "set": "y",
        "value": 2,
      }],
    }],
    "else": [{
      "set": "y",
      "value": 3,
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, 3);
});

test("031", () => {
  const c = run([{
    "set": "x",
    "value": {
      "unop": "-",
      "arg": 500,
    },
  }, {
    "if": [{
      "cond": {
        "binop": ">",
        "argl": "x",
        "argr": 0,
      },
      "then": [{
        "set": "y",
        "value": 1,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 100,
        },
      },
      "then": [{
        "set": "y",
        "value": 2,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 50,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 50,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 60,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 60,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 65,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 65,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 75,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 75,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 90,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 90,
        },
      }],
    }],
    "else": [{
      "set": "y",
      "value": 3,
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, 2);
});

test("033", () => {
  const c = run([{
    "set": "x",
    "value": {
      "unop": "-",
      "arg": 95,
    },
  }, {
    "if": [{
      "cond": {
        "binop": ">",
        "argl": "x",
        "argr": 0,
      },
      "then": [{
        "set": "y",
        "value": 1,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 100,
        },
      },
      "then": [{
        "set": "y",
        "value": 2,
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 50,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 50,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 60,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 60,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 65,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 65,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 75,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 75,
        },
      }],
    }, {
      "cond": {
        "binop": "<",
        "argl": "x",
        "argr": {
          "unop": "-",
          "arg": 90,
        },
      },
      "then": [{
        "set": "y",
        "value": {
          "unop": "-",
          "arg": 90,
        },
      }],
    }],
    "else": [{
      "set": "y",
      "value": 3,
    }],
  }, {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, -50);
});

test("006", () => {
  const c = run([{
    "set": "out",
    "value": {
      "binop": "|",
      "argl": {
        "binop": "+",
        "argl": {
          "binop": "+",
          "argl": 1,
          "argr": {
            "binop": "*",
            "argl": 2,
            "argr": 5,
          },
        },
        "argr": 4,
      },
      "argr": {
        "binop": "&",
        "argl": {
          "binop": "+",
          "argl": 12,
          "argr": 33,
        },
        "argr": {
          "binop": ">>",
          "argl": {
            "binop": "-",
            "argl": 256,
            "argr": 4,
          },
          "argr": {
            "binop": "-",
            "argl": {
              "binop": "*",
              "argl": 2,
              "argr": 3,
            },
            "argr": {
              "binop": "or",
              "argl": {
                "binop": "and",
                "argl": {
                  "binop": ">",
                  "argl": 4,
                  "argr": 8,
                },
                "argr": 1,
              },
              "argr": 0,
            },
          },
        },
      },
    },
  }]);
  assertEquals(c, 15);
});

test("001", () => {
  const c = run([{
    "set": "out",
    "value": 1,
  }]);
  assertEquals(c, 1);
});

test("003", () => {
  const c = run([{
    "set": "out",
    "value": {
      "binop": "and",
      "argl": {
        "binop": "|",
        "argl": {
          "binop": "+",
          "argl": {
            "binop": "+",
            "argl": 1,
            "argr": {
              "binop": "*",
              "argl": 2,
              "argr": 5,
            },
          },
          "argr": 4,
        },
        "argr": {
          "binop": "&",
          "argl": {
            "binop": "+",
            "argl": 12,
            "argr": 33,
          },
          "argr": 256,
        },
      },
      "argr": {
        "binop": "<<",
        "argl": 4,
        "argr": {
          "binop": "*",
          "argl": 2,
          "argr": 3,
        },
      },
    },
  }]);
  assertEquals(c, 256);
});

test("009", () => {
  const c = run([{
    "set": "out",
    "value": {
      "binop": "^",
      "argl": 7,
      "argr": {
        "binop": "^",
        "argl": 7,
        "argr": {
          "unop": "-",
          "arg": {
            "binop": "^",
            "argl": {
              "binop": "/",
              "argl": 1,
              "argr": 2,
            },
            "argr": 2,
          },
        },
      },
    },
  }]);
  assertAlmostEquals(c, 3.3079296368936, 1e-5);
});

test("002", () => {
  const c = run([{
    "set": "out",
    "value": {
      "binop": "+",
      "argl": 1,
      "argr": 2,
    },
  }]);
  assertEquals(c, 3);
});

test("007", () => {
  const c = run([{
    "set": "out",
    "value": {
      "binop": "or",
      "argl": {
        "binop": "and",
        "argl": {
          "binop": ">",
          "argl": {
            "binop": "|",
            "argl": {
              "binop": "+",
              "argl": {
                "binop": "+",
                "argl": 1,
                "argr": {
                  "binop": "*",
                  "argl": 2,
                  "argr": 5,
                },
              },
              "argr": 4,
            },
            "argr": {
              "binop": "&",
              "argl": {
                "binop": "+",
                "argl": 12,
                "argr": 33,
              },
              "argr": {
                "binop": ">>",
                "argl": {
                  "binop": "-",
                  "argl": 256,
                  "argr": 4,
                },
                "argr": {
                  "binop": "-",
                  "argl": {
                    "binop": "*",
                    "argl": 2,
                    "argr": 3,
                  },
                  "argr": 4,
                },
              },
            },
          },
          "argr": 8,
        },
        "argr": 1,
      },
      "argr": 0,
    },
  }]);
  assertEquals(c, 1);
});

test("005", () => {
  const c = run([{
    "set": "out",
    "value": {
      "binop": "or",
      "argl": {
        "binop": "or",
        "argl": {
          "binop": "|",
          "argl": {
            "binop": "+",
            "argl": {
              "binop": "+",
              "argl": 1,
              "argr": {
                "binop": "*",
                "argl": 2,
                "argr": 5,
              },
            },
            "argr": 4,
          },
          "argr": {
            "binop": "&",
            "argl": {
              "binop": "+",
              "argl": 12,
              "argr": 33,
            },
            "argr": 256,
          },
        },
        "argr": {
          "binop": ">>",
          "argl": 4,
          "argr": {
            "binop": "*",
            "argl": 2,
            "argr": 3,
          },
        },
      },
      "argr": {
        "binop": ">",
        "argl": 4,
        "argr": 8,
      },
    },
  }]);
  assertEquals(c, 15);
});

test("004", () => {
  const c = run([{
    "set": "out",
    "value": {
      "binop": "or",
      "argl": {
        "binop": "|",
        "argl": {
          "binop": "+",
          "argl": {
            "binop": "+",
            "argl": 1,
            "argr": {
              "binop": "*",
              "argl": 2,
              "argr": 5,
            },
          },
          "argr": 4,
        },
        "argr": {
          "binop": "+",
          "argl": 12,
          "argr": 33,
        },
      },
      "argr": {
        "binop": "<<",
        "argl": 4,
        "argr": 2,
      },
    },
  }]);
  assertEquals(c, 47);
});

test("010", () => {
  const c = run([{
    "set": "out",
    "value": {
      "binop": "-",
      "argl": 2,
      "argr": {
        "unop": "-",
        "arg": {
          "unop": "-",
          "arg": 2,
        },
      },
    },
  }]);
  assertEquals(c, 0);
});

test("008", () => {
  const c = run([{
    "set": "out",
    "value": {
      "binop": "^",
      "argl": 2,
      "argr": {
        "binop": "^",
        "argl": 3,
        "argr": 2,
      },
    },
  }]);
  assertEquals(c, 512.0);
});

test("049.semi", () => {
  const c = run([{
    "function": "add",
    "args": ["x", "y"],
    "block": [{
      "set": "z",
      "value": 0,
    }, {
      "iterator": "i",
      "from": 0,
      "to": "x",
      "do": [{
        "set": "z",
        "value": {
          "call": "inc",
          "args": ["z"],
        },
      }],
    }, {
      "return": "z",
    }],
  }, {
    "function": "inc",
    "args": ["x"],
    "block": [{
      "return": {
        "binop": "+",
        "argl": "x",
        "argr": 1,
      },
    }],
  }, {
    "set": "out",
    "value": {
      "call": "add",
      "args": [4, 3],
    },
  }]);
  assertEquals(c, 5);
});

test("047", () => {
  const c = run([{
    "function": "inc",
    "args": ["x"],
    "block": [{
      "return": {
        "binop": "+",
        "argl": "x",
        "argr": 1,
      },
    }],
  }, {
    "set": "out",
    "value": {
      "call": "inc",
      "args": [10],
    },
  }]);
  assertEquals(c, 11);
});

test("050", () => {
  const c = run([{
    "function": "inc",
    "args": ["x"],
    "block": [{
      "return": {
        "binop": "+",
        "argl": "x",
        "argr": 1,
      },
    }],
  }, {
    "function": "add",
    "args": ["x", "y"],
    "block": [{
      "set": "z",
      "value": 0,
    }, {
      "iterator": "i",
      "from": 0,
      "to": "x",
      "do": [{
        "set": "z",
        "value": {
          "call": "inc",
          "args": ["z"],
        },
      }],
    }, {
      "return": "z",
    }],
  }, {
    "set": "out",
    "value": {
      "binop": "+",
      "argl": {
        "call": "add",
        "args": [4, 3],
      },
      "argr": {
        "call": "inc",
        "args": [1],
      },
    },
  }]);
  assertEquals(c, 7);
});

test("051.semi", () => {
  const c = run([{
    "function": "add_rec",
    "args": ["x", "y"],
    "block": [{
      "if": [{
        "cond": {
          "binop": "==",
          "argl": "x",
          "argr": 0,
        },
        "then": [{
          "return": "y",
        }],
      }],
      "else": [{
        "return": {
          "call": "add_rec",
          "args": ["y", {
            "binop": "-",
            "argl": "x",
            "argr": 1,
          }],
        },
      }],
    }],
  }, {
    "set": "out",
    "value": {
      "call": "add_rec",
      "args": [4, 3],
    },
  }]);
  assertEquals(c, 0);
});

test("052", () => {
  const c = run([{
    "set": "x",
    "value": 0,
  }, {
    "function": "make_it_six",
    "args": [],
    "block": [{
      "set": "x",
      "value": {
        "binop": "+",
        "argl": "x",
        "argr": 1,
      },
    }, {
      "if": [{
        "cond": {
          "binop": "<",
          "argl": "x",
          "argr": 6,
        },
        "then": [{
          "call": "make_it_six",
          "args": [],
        }],
      }],
    }, {
      "return": "x",
    }],
  }, {
    "set": "out",
    "value": {
      "call": "make_it_six",
      "args": [],
    },
  }]);
  assertEquals(c, 6);
});

test("048", () => {
  const c = run([{
    "function": "inc",
    "args": ["x"],
    "block": [{
      "return": {
        "binop": "+",
        "argl": "x",
        "argr": 1,
      },
    }],
  }, {
    "function": "add",
    "args": ["x", "y"],
    "block": [{
      "set": "z",
      "value": 0,
    }, {
      "iterator": "i",
      "from": 0,
      "to": "x",
      "do": [{
        "set": "z",
        "value": {
          "call": "inc",
          "args": ["z"],
        },
      }],
    }, {
      "return": "z",
    }],
  }, {
    "set": "out",
    "value": {
      "call": "add",
      "args": [4, 3],
    },
  }]);
  assertEquals(c, 5);
});

test("020", () => {
  const c = run([{
    "set": "x",
    "value": 1,
  }, [{
    "set": "x",
    "value": 2,
  }, [{
    "set": "x",
    "value": 3,
  }]], {
    "set": "out",
    "value": "x",
  }]);
  assertEquals(c, 3);
});

test("018", () => {
  const c = run([{
    "set": "x",
    "value": 1,
  }, [{
    "declare": "x",
    "value": 2,
  }, [{
    "set": "x",
    "value": 3,
  }], {
    "set": "out",
    "value": "x",
  }]]);
  assertEquals(c, 3);
});

test("017", () => {
  const c = run([{
    "set": "x",
    "value": 1,
  }, [{
    "declare": "x",
    "value": 2,
  }], {
    "set": "out",
    "value": "x",
  }]);
  assertEquals(c, 1);
});

test("019", () => {
  const c = run([{
    "set": "x",
    "value": 1,
  }, [{
    "declare": "x",
    "value": 2,
  }, [{
    "set": "x",
    "value": 3,
  }]], {
    "set": "out",
    "value": "x",
  }]);
  assertEquals(c, 1);
});

test("021", () => {
  const c = run([{
    "set": "x",
    "value": 1,
  }, [{
    "declare": "x",
    "value": 2,
  }, [{
    "declare": "x",
    "value": 3,
  }]], {
    "set": "out",
    "value": "x",
  }]);
  assertEquals(c, 1);
});

test("015", () => {
  const c = run([{
    "set": "x",
    "value": 1,
  }, [{
    "set": "x",
    "value": 2,
  }, {
    "set": "y",
    "value": 4,
  }], {
    "set": "out",
    "value": "x",
  }]);
  assertEquals(c, 2);
});

test("016", () => {
  const c = run([{
    "set": "x",
    "value": 1,
  }, [{
    "set": "x",
    "value": 2,
  }, {
    "set": "y",
    "value": 4,
  }], {
    "set": "out",
    "value": "y",
  }]);
  assertEquals(c, 4);
});

test("023", () => {
  const c = run([[{
    "set": "x",
    "value": 2,
  }, [{
    "declare": "x",
    "value": 3,
  }]], {
    "set": "out",
    "value": "x",
  }]);
  assertEquals(c, 2);
});

cp.execSync("rm test.jsonlang test.wasm test.wat");
console.log(`Passed ${totalPass}/${totalTests}`);
process.exit(totalPass == totalTests ? 0 : 1);