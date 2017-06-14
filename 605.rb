def can_place_flowers(flowerbed, n)
  x = 0
  possible = 0

  while x < flowerbed.count
    l = x-1 < 0 ? true : (flowerbed[x - 1] != 1)
    r = x+1 > flowerbed.length - 1 ? true : (flowerbed[x + 1] != 1)

    if l && r && flowerbed[x] == 0
      possible += 1
      flowerbed[x] = 1
    end

    x += 1
  end

  n <= possible
end