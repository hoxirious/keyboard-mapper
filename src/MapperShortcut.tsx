import { ButtonShortcut, MapType } from "./ButtonShortcut"
import "./styles/components.scss"
import { useEffect } from "react"

interface MapperShortcutProps {
    keybind_id: number;
    mapfrom: string;
    mapto: string;
}

export const MapperShortcut = ({keybind_id, mapfrom, mapto}: MapperShortcutProps) => {

    return (
        <div className="mapper-shortcut">
            <ButtonShortcut id={keybind_id} keybind={mapfrom} mapType={MapType.MapFrom} />
            <p>to</p>
            <ButtonShortcut id={keybind_id} keybind={mapto} mapType={MapType.MapTo} />
        </div>
    )
}
