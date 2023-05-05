
const indentText = (text, indentLevel) => {
    for (let i = 0; i < indentLevel; i++) {
        text = " " + text
    }

    return text
}

const convertValue = (val, indentLevel, arrayItem = false) => {
    if (typeof val === "string") {
        // if (arrayItem) {
        //     return indentText(`- "${val}"`, indentLevel)
        // }

        return `"${val}"`
    }

    if (val instanceof Array) {
        const items = []

        for (const v of val) {
            if (typeof v === "string") {
                items.push(indentText("- " + v, indentLevel + 2))

                continue
            }

            if (typeof v === "object") {
                items.push(convertObject(v, indentLevel + 2, true))

                continue
            }

            items.push(convertValue(v, indentLevel + 2, true))
        }

        return "\n" + items.join("\n")
    }

    if (typeof val === "object") {
        return `\n${convertObject(val, indentLevel + 2, arrayItem)}`
    }

    return `${val}`
}

const convertField = (key, val, intentLevel, arrayItem = false) => {
    const indent = arrayItem ? "- " : ""

    return `${key}: ${convertValue(val, intentLevel)}`
}

const convertObject = (obj, indentLevel = 0, arrayItem = false) => {
    let first = true

    const items = []

    for (const [key, val] of Object.entries(obj)) {
        const arrayIntent = arrayItem ?
            first ?
                "- " :
                "  " :
            ""


        items.push(
            indentText(arrayIntent + convertField(key, val, indentLevel, first && arrayItem), indentLevel))

        first = false
    }

    return items.join("\n")
}

export const toYML = (obj: any) => {
    return convertObject(obj)
}