import React from "react";

// The offset makes the center of the image be the center of the canvas element.
type ResetMapComponentProps = {
    reset_parent_func: () => any;
};
export const ResetMapComponent = ({ reset_parent_func }: ResetMapComponentProps) => {
    return (
        <button onClick={reset_parent_func}>
            Reset Map
        </button>
    )
}
