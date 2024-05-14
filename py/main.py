import os
import sys


def rm_extra_chars(src):
    lang = "+-<>.,[]"
    return list(filter(lambda x: x in lang, src))


def run(src: str, step=True):
    src = rm_extra_chars(src)
    i = 0
    loop_i = 0
    cursor = 0
    state = [0 for _ in range(1024)]
    out = []

    while i < len(src):
        c = src[i]
        match c:
            case "+":
                state[cursor] += 1
            case "-":
                state[cursor] -= 1
            case ">":
                cursor += 1
            case "<":
                cursor -= 1
            case ".":
                out.append(chr(state[cursor]))
                if not step:
                    print("\033c")
                    print("".join(o for o in out))
            case ",":
                pass
                # user input
            case "[":
                loop_i = i
            case "]":
                if state[cursor] > 0:
                    i = loop_i
        if step:
            print(state)
            print("".join(c for c in out))
            x = input()
            if x == "q":
                exit()
            print("\033c")
        i += 1


if __name__ == "__main__":
    if len(sys.argv) < 2:
        exit(1)
    elif not os.path.exists(sys.argv[1]):
        print(sys.argv[1], "does not exist")
        exit(1)

    with open(sys.argv[1]) as f:
        src = f.read()

    print(src)
    print("*********")
    # run(src, step=False)
    run(src, step=False)
