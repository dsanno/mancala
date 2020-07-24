import pprint

PIT_NUM = 6
MAX_SEED = 48
PIT_BITS_WIDTH = 8
CYCLE_SIZE = PIT_NUM * 2 + 1


def main():
    bits_diffs = []
    opponent_bits_diffs = []
    stores = []
    for i in range(PIT_NUM):
        pit_bits_diffs = [0]
        pit_opponent_bits_diffs = [0]
        pit_stores = [0]
        for j in range(1, MAX_SEED + 1):
            diffs = [0] * CYCLE_SIZE
            diffs[i] = -j
            for k in range(j):
                diffs[(i + k + 1) % CYCLE_SIZE] += 1
            pit_bits_diffs.append(_diffs_to_bits(diffs[:PIT_NUM]))
            pit_opponent_bits_diffs.append(_diffs_to_bits(diffs[PIT_NUM + 1:]))
            pit_stores.append(diffs[PIT_NUM])
        bits_diffs.append(pit_bits_diffs)
        opponent_bits_diffs.append(pit_opponent_bits_diffs)
        stores.append(pit_stores)

    width = 115
    pprint.pprint(bits_diffs, width=width, compact=True)
    pprint.pprint(opponent_bits_diffs, width=width, compact=True)
    pprint.pprint(stores, width=width, compact=True)


def _diffs_to_bits(diffs):
    bits = 0
    for diff in reversed(diffs):
        bits = (bits << PIT_BITS_WIDTH) + diff
    return bits


if __name__ == '__main__':
    main()
