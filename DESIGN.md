t = ticks - 1/8th of a second.
i = items

smallest claw moves 1i/8t, then 1i/16t, 1i/24t, etc.
conveyor moves 1i/4t (900 per day)

 3600t:  1 day (7.5 real-life minutes)
    4t:  900 per day (1 per irl 0.5s)
    8t:  450 per day (1 per irl 1s)
   16t:  225 per day (1 per irl 2s)
   40t:  90 per day (1 per irl 5s)
   60t:  60 per day (1 per irl 7.5s)
   80t:  45 per day (1 per irl 10s)
   120t: 30 per day (1 per irl 15s)

Elements:
    A - Animus (4kg/1L)
    E - Earth (2kg/1L)
    Fe - Ferrous (8kg/1L)
    I - Impurity (6kg/2L)
    R - Rock (4kg/1L)

Item:
    The volume of an item is 2L + the sum of the components' volumes.
    IFeI - Magnetite
    Fe - Pure Ferrous
    FeFe - Iron Lump
    RR - Iron Precursor
    EE - Rock Precursor
    IAI - Animite
    A - Pure Animus

Buildings:
    Conveyor:
        1x Iron Lump, 1x Pure Animus
        Is a container
        Moves items between other conveyors
    Purifier:
        6x Iron Lump, 1x Pure Animus
        Removes all impurities after 40t
        E.G. putting in Magnetite results in a Pure Ferrous
    Merger:
        4x Iron Lump, 2x Pure Animus
        Combines lists of elements after 20t
        E.G. putting in two Pure Ferrous results in an Iron Lump
    Claw:
        1x Iron Lump, 3x Pure Animus + 1x Iron Lump per length
        Moves items between containers
    Sm. Warehouse:
        10x Iron Lump
        Stores 20,000L of items (e.g. 5,000 iron lumps)
    Sm. Silo:
        5x Iron Lump
        Stores 20,000L of a single item

Transmutations:
    RR -> Fe
    EE -> R
