import argparse
import struct

import numpy as np
import torch


VALUE_PER_SEED = 100
PATTERN_NUM = 60
PATTERN_SIZE = 15 * 15 * 15
SEED_TO_INDEX_0 = np.array([
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
], dtype=np.int32)
SEED_TO_INDEX_1 = np.array([
    0, 15, 30, 45, 60, 75, 90, 105, 120, 135, 150, 165, 180, 195, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210,
    210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210,
    210,
], dtype=np.int32)
SEED_TO_INDEX_2 = np.array([
    0, 225, 450, 675, 900, 1125, 1350, 1575, 1800, 2025, 2250, 2475, 2700, 2925, 3150, 3150, 3150, 3150, 3150, 3150,
    3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150,
    3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150,
], dtype=np.int32)


class Evaluator(torch.nn.Module):

    def __init__(self):
        super(Evaluator, self).__init__()
        embeddings = [torch.nn.Embedding(PATTERN_SIZE, 1) for _ in range(PATTERN_NUM)]
        for embedding in embeddings:
            torch.nn.init.normal_(embedding.weight, mean=0, std=1)
        self.embeddings = torch.nn.ModuleList(embeddings)

    def forward(self, x):
        y = 0
        for i, embedding in zip(range(x.shape[1]), self.embeddings):
            y += embedding(x[:, i])
        return y


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('position_path', help='Position data file path')
    parser.add_argument('output_path', help='Output file path')
    return parser.parse_args()


def main():
    args = parse_args()
    patterns, values = load_patterns(args.position_path)
    network = Evaluator()
    train(patterns, values, network, )
    values = [embedding.weight.detach().numpy().ravel() for embedding in network.embeddings]
    save_values(args.output_path, values)


def load_patterns(file_path):
    patterns = []
    values = []
    with open(file_path, 'rb') as f:
        _, record_num = struct.unpack('<ii', f.read(8))
        for i in range(record_num):
            raw_bytes = f.read(24)
            seeds = struct.unpack('BBBBBBBBBBBBBBBB', raw_bytes[0:16])
            value, visit = struct.unpack('<ii', raw_bytes[16:24])
            if visit < 10 and np.random.random() * 10 >= visit:
                continue
            pattern = make_pattern(np.array(seeds[0:6], dtype=np.int32), np.array(seeds[8:14], dtype=np.int32))
            patterns.append(pattern)
            values.append([value])
            if (i + 1) % 10000 == 0:
                print('{} / {} loaded'.format(i + 1, record_num))
        print('number of positions:', len(values))
    return torch.LongTensor(patterns), torch.tensor(values, dtype=torch.float32)


def save_values(file_path, values_list):
    with open(file_path, 'wb') as f:
        for values in values_list:
            for value in values:
                f.write(struct.pack('<i', int(value)))


def train(inputs, targets, network, epoch_num=100, batch_size=128):
    dataset = torch.utils.data.TensorDataset(inputs, targets)
    data_loader = torch.utils.data.DataLoader(dataset, batch_size=batch_size, shuffle=True)
    optimizer = torch.optim.SGD(network.parameters(), lr=0.01, momentum=0.9)

    print('start training')
    for epoch in range(epoch_num):
        accumulate_loss = 0
        accumulate_num = 0
        for x, t in data_loader:
            y = network(x)
            loss = torch.nn.functional.mse_loss(y, t)
            optimizer.zero_grad()
            loss.backward()
            optimizer.step()
            accumulate_num += x.shape[0]
            accumulate_loss += loss.detach().numpy() * x.shape[0]
        print('Epoch {} loss: {}'.format(epoch, accumulate_loss / accumulate_num))
    print('done')


def make_pattern(x, y):
    return [
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[x[1]] + SEED_TO_INDEX_2[y[5]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[x[1]] + SEED_TO_INDEX_2[y[4]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[y[5]] + SEED_TO_INDEX_2[y[4]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[y[5]] + SEED_TO_INDEX_2[y[4]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[x[2]] + SEED_TO_INDEX_2[y[5]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[x[2]] + SEED_TO_INDEX_2[y[3]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[y[5]] + SEED_TO_INDEX_2[y[3]],
        SEED_TO_INDEX_0[x[2]] + SEED_TO_INDEX_1[y[5]] + SEED_TO_INDEX_2[y[3]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[x[3]] + SEED_TO_INDEX_2[y[5]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[x[3]] + SEED_TO_INDEX_2[y[2]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[y[5]] + SEED_TO_INDEX_2[y[2]],
        SEED_TO_INDEX_0[x[3]] + SEED_TO_INDEX_1[y[5]] + SEED_TO_INDEX_2[y[2]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[x[4]] + SEED_TO_INDEX_2[y[5]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[x[4]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[y[5]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[4]] + SEED_TO_INDEX_1[y[5]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[x[5]] + SEED_TO_INDEX_2[y[5]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[x[5]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[0]] + SEED_TO_INDEX_1[y[5]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[5]] + SEED_TO_INDEX_1[y[5]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[x[2]] + SEED_TO_INDEX_2[y[4]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[x[2]] + SEED_TO_INDEX_2[y[3]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[y[4]] + SEED_TO_INDEX_2[y[3]],
        SEED_TO_INDEX_0[x[2]] + SEED_TO_INDEX_1[y[4]] + SEED_TO_INDEX_2[y[3]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[x[3]] + SEED_TO_INDEX_2[y[4]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[x[3]] + SEED_TO_INDEX_2[y[2]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[y[4]] + SEED_TO_INDEX_2[y[2]],
        SEED_TO_INDEX_0[x[3]] + SEED_TO_INDEX_1[y[4]] + SEED_TO_INDEX_2[y[2]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[x[4]] + SEED_TO_INDEX_2[y[4]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[x[4]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[y[4]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[4]] + SEED_TO_INDEX_1[y[4]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[x[5]] + SEED_TO_INDEX_2[y[4]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[x[5]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[1]] + SEED_TO_INDEX_1[y[4]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[5]] + SEED_TO_INDEX_1[y[4]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[2]] + SEED_TO_INDEX_1[x[3]] + SEED_TO_INDEX_2[y[3]],
        SEED_TO_INDEX_0[x[2]] + SEED_TO_INDEX_1[x[3]] + SEED_TO_INDEX_2[y[2]],
        SEED_TO_INDEX_0[x[2]] + SEED_TO_INDEX_1[y[3]] + SEED_TO_INDEX_2[y[2]],
        SEED_TO_INDEX_0[x[3]] + SEED_TO_INDEX_1[y[3]] + SEED_TO_INDEX_2[y[2]],
        SEED_TO_INDEX_0[x[2]] + SEED_TO_INDEX_1[x[4]] + SEED_TO_INDEX_2[y[3]],
        SEED_TO_INDEX_0[x[2]] + SEED_TO_INDEX_1[x[4]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[2]] + SEED_TO_INDEX_1[y[3]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[4]] + SEED_TO_INDEX_1[y[3]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[2]] + SEED_TO_INDEX_1[x[5]] + SEED_TO_INDEX_2[y[3]],
        SEED_TO_INDEX_0[x[2]] + SEED_TO_INDEX_1[x[5]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[2]] + SEED_TO_INDEX_1[y[3]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[5]] + SEED_TO_INDEX_1[y[3]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[3]] + SEED_TO_INDEX_1[x[4]] + SEED_TO_INDEX_2[y[2]],
        SEED_TO_INDEX_0[x[3]] + SEED_TO_INDEX_1[x[4]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[3]] + SEED_TO_INDEX_1[y[2]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[4]] + SEED_TO_INDEX_1[y[2]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[3]] + SEED_TO_INDEX_1[x[5]] + SEED_TO_INDEX_2[y[2]],
        SEED_TO_INDEX_0[x[3]] + SEED_TO_INDEX_1[x[5]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[3]] + SEED_TO_INDEX_1[y[2]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[5]] + SEED_TO_INDEX_1[y[2]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[4]] + SEED_TO_INDEX_1[x[5]] + SEED_TO_INDEX_2[y[1]],
        SEED_TO_INDEX_0[x[4]] + SEED_TO_INDEX_1[x[5]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[4]] + SEED_TO_INDEX_1[y[1]] + SEED_TO_INDEX_2[y[0]],
        SEED_TO_INDEX_0[x[5]] + SEED_TO_INDEX_1[y[1]] + SEED_TO_INDEX_2[y[0]],
    ]


if __name__== '__main__':
    main()
