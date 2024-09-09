
def layersort (layers: list[tuple[float]]) -> list[list[tuple[float]]]:
    '''
    layers is list of size 2 tuples of floats
    first element is the start of the layer, second is the end
    '''
    layers = sorted(layers, key=lambda x: (x[0], -x[1]))
    groups = []
    maxlens = {}
    for layer in layers:
        added = False
        for idx, group in enumerate(groups):
            if group[-1][1] < layer[0]:
                group.append(layer)
                maxlens[idx] = max(layer[1] - layer[0], maxlens[idx])
                added = True
                break
        if not added:
            groups.append([layer])
            maxlens[len(groups) - 1] = layer[1] - layer[0]
    # print(groups)
    # print(maxlens)

    group_order = sorted(range(len(groups)), key=lambda x: -maxlens[x])
    # print(group_order)

    out_groups = []
    for i in group_order:
        out_groups.append(groups[i])

    return out_groups


def test():
    layers = [
        (10, 30),
        (21, 31),
        (14, 20),
        (19, 30),
        (35, 36)
    ]

    out = layersort(layers)

    for i in out:
        print(i)

if __name__ == '__main__':
    test()