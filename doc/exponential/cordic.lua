-- lua source code for the cordic-based pow polyfill --
-- jsonlang does not support arrays --
function cordic_lut(n)
  if     n == 0  then return 1.648721270700128
  elseif n == 1  then return 1.284025416687742
  elseif n == 2  then return 1.133148453066826
  elseif n == 3  then return 1.064494458917859
  elseif n == 4  then return 1.031743407499103
  elseif n == 5  then return 1.015747708586686
  elseif n == 6  then return 1.007843097206488
  elseif n == 7  then return 1.003913889338348
  elseif n == 8  then return 1.001955033591003
  elseif n == 9  then return 1.000977039492417
  elseif n == 10 then return 1.000488400478694
  elseif n == 11 then return 1.000244170429748
  elseif n == 12 then return 1.000122077763384
  elseif n == 13 then return 1.000061037018933
  elseif n == 14 then return 1.000030518043791
  elseif n == 15 then return 1.0000152589054785
  elseif n == 16 then return 1.0000076294236351
  elseif n == 17 then return 1.0000038147045416
  elseif n == 18 then return 1.0000019073504518
  elseif n == 19 then return 1.0000009536747712
  elseif n == 20 then return 1.0000004768372719
  elseif n == 21 then return 1.0000002384186075
  elseif n == 22 then return 1.0000001192092967
  elseif n == 23 then return 1.0000000596046466
  elseif n == 24 then return 1.0000000298023228
  end
end

-- dummy, replace with wasm floor unop
function floor(n)
  return math.floor(n)
end

function ln_cordic(x)
  local result = 0
  local e = 2
  while e <= x do
    result = result + 1
    x = x / e
  end
  while x < 1 do
    result = result - 1
    x = x * e
  end
  local w = 0
  local power = (1/2)
  for i = 0, 24 do
    if cordic_lut(i) < x then
      result = result + power
      x = x / cordic_lut(i)
    end
    power = power / 2
  end
  x = x - 1
  x = x * (1 - (x/2)) * (1 + (x/3)) * (1 - (x/4))
  return result + x
end

function exp_cordic(x)
  local e = 2
  local int_part = floor(x)
  local result = 1
  while int_part > 0 do
    int_part = int_part - 1
    result = result * e
  end
  while int_part < 0 do
    int_part = int_part + 1
    result = result / e
  end
  local z = x - floor(x)
  local power = (1/2)
  for i = 0, 24 do
    if power < z then
      result = result * cordic_lut(i)
      z = z - power
    end
    power = power / 2
  end
  result = result * (1 + z * (1 + (z/2) * (1 + (z/3) * (1 + (z/4)))))
  return result
end

function pow(base, exponent)
  local result = 1
  while exponent >= 1 do
    exponent = exponent - 1
    result = result * base
  end
  while exponent < 0 do
    exponent = exponent + 1
    result = result / base
  end
  if exponent ~= 0 then
    return result * exp_cordic(ln_cordic(base)*exponent)
  end
  return result
end

