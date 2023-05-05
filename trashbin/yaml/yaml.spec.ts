import { toYML } from "./yml"

it("convert simple object to yml", () => {
    const y = toYML({
        name: "matti",
        age: 23
    })

    expect(y).toBe(`name: "matti"
age: 23`)
})

it("convert object children to yml", () => {
    const y = toYML({
        name: "matti",
        age: 24,
        boss: {
            name: "jorma"
        }
    })

    expect(y).toBe(`name: "matti"
age: 24
boss: 
  name: "jorma"`)
})

it("convert object with array child to yml", () => {
    const y = toYML({
        name: "matti",
        friends: [
            {
                name: "teppo",
                age: 4
            }
        ]
    })

    expect(y).toBe(`name: "matti"
friends: 
  - name: "teppo"
    age: 4`)
})

it("Enum values work correctly", () => {
    const y = toYML({
        name: "response",
        state: ["SUCCESS", "FAILED"]
    })

    console.log(y)

    expect(y).toBe(`name: "response"
state:
  - SUCCESS
  - FAILED`)
})