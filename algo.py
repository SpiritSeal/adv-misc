

def layersort (layers: list[tuple[float]]) -> list[list[tuple[float]]]:
    '''
    layers is list of size 2 tuples of floats
    first element is the start of the layer, second is the end
    sorting prio is that layers with greatest length are first
    if multiple layers are non-overlapping, move them into the same group
    '''
    # sort by length
    layers = sorted(layers, key=lambda x: x[1] - x[0], reverse=True)
    # sort by start time
    layers = sorted(layers, key=lambda x: x[0])
    # group layers
    groups = []
    for layer in layers:
        added = False
        for group in groups:
            if group[-1][1] < layer[0]:
                group.append(layer)
                added = True
                break
        if not added:
            groups.append([layer])
    return groups



def test():
    layers = [
        (10, 30),
        (21, 31),
        (14, 20),
        (19, 22),
        (35, 36)
    ]

    out = layersort(layers)

    for i in out:
        print(i)

if __name__ == '__main__':
    test()