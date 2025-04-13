import { SIZES, type Size } from "$lib/types/sizes";

export const convert_size_to_fontsize = (size: Size) => {
    return `${SIZES[size]* 2.2}rem`;
}

export const get_color_for_tag = (tag: string) => {
    switch (tag) {
        case "RPG":
            return "bg-red-500";
        case "Platformer":
            return "bg-blue-500";
        default:
            return "bg-gray-500";
    }
}