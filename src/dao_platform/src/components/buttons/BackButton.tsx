import { ArrowLeft } from "lucide-react";
import { Button } from "@/components/ui/button";

type BackButtonProps = {
    title?: string;
    onBack: () => void;
}

export const BackButton = ({ onBack, title = 'Back' }: BackButtonProps) => {
    return (
        <div className="mb-6">
            <Button
                variant="ghost"
                onClick={onBack}
                className="flex items-center gap-2"
            >
                <ArrowLeft className="h-4 w-4" />
                {title}
            </Button>
        </div>
    );
}