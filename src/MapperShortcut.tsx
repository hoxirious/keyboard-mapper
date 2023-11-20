import { ButtonShortcut, MapType } from "./ButtonShortcut"
import "./styles/components.scss"
import { useEffect } from "react"

interface MapperShortcutProps {
    mapfrom: string;
    mapto: string;
}

export const MapperShortcut = ({mapfrom, mapto}: MapperShortcutProps) => {

    return (
        <div className="mapper-shortcut">
            <ButtonShortcut keybind={mapfrom} mapType={MapType.MapFrom} />
            <p>to</p>
            <ButtonShortcut keybind={mapto} mapType={MapType.MapTo} />
        </div>
    )
}
