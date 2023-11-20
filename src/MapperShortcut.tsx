import { ButtonShortcut } from "./ButtonShortcut"
import "./styles/components.scss"
import { useEffect } from "react"

interface MapperShortcutProps {
    mapfrom: string;
    mapto: string;
}

export const MapperShortcut = ({mapfrom, mapto}: MapperShortcutProps) => {

    return (
        <div className="mapper-shortcut">
            <ButtonShortcut keybind={mapfrom} />
            <p>to</p>
            <ButtonShortcut keybind={mapto} />
        </div>
    )
}
