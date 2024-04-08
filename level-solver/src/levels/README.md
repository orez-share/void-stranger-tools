# Level Format

**This article contains spoilers for Void Stranger. You should go play it before reading this.**

Levels are stored and displayed graphically; you can see some examples in this directory.

Levels are composed of **tiles** and **entities**.
Usually, tiles are represented by symbols, and entities are represented by alphanum.

Entities are assumed to start on plain floor tiles.
Entities which do not start on floor tiles may be denoted by combining marks.

eg:
- `@` - a player on floor
- `@̟` - a player on a trap

### Tiles
| Symbol | Combo | Name |
| --- | --- | --- |
| `.` | *\<default>* | floor |
| `#` | *\<none>* | wall |
| ` ` | *\<none>* | hole |
| `+` | ◌̟ | trap |
| `!` | ◌̟ | primed trap |
| `>` | ◌͕ | exit |
| `x` | ◌͓ | button |
| `^` | ◌̭ | glass |
| `%` | *\<none>* | death floor |

### Entities
| Symbol | Name |
| --- | --- |
| `@` | player |
| `8` | covetous statue |
| `7` | devious statue |
| `5` | raging statue |
| `q` | golem |
| `p` | golem (x-mirror) |
| `d` | golem (y-mirror) |
| `b` | golem (xy-mirror) |
| `H` | hand |
| `o` | egg |
| `K` | bull |
| `e` | teeth |
