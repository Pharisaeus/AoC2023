import z3


def main():
    content = open("24.txt", 'r').read()
    hails = []
    for line in content.split("\n"):
        pos, vel = line.strip().split(" @ ")
        pos = [int(c) for c in pos.split(", ")]
        vel = [int(c) for c in vel.split(", ")]
        hails.append((pos, vel))
    s = z3.Solver()
    pos = [z3.Real("px"), z3.Real("py"), z3.Real("pz")]
    vel = [z3.Real("vx"), z3.Real("vy"), z3.Real("vz")]
    t = [z3.Real("t" + str(i)) for i in range(len(hails))]
    for i, (hail_pos, hail_vel) in enumerate(hails):
        x, y, z = hail_pos
        vxx, vyy, vzz = hail_vel
        s.add(pos[0] + vel[0] * t[i] == x + vxx * t[i])
        s.add(pos[1] + vel[1] * t[i] == y + vyy * t[i])
        s.add(pos[2] + vel[2] * t[i] == z + vzz * t[i])
    model = s.model()
    print(sum([model[p].as_long() for p in pos]))


main()
