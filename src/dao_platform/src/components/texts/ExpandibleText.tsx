import { useState } from "react";
import { TextModal } from "../modals/TextModal";
import { Eye } from "lucide-react";

interface ExpandibleTextProps {
    text: string;
    maxLength?: number;
}

export const ExpandibleText = ({ text, maxLength = 150 }: ExpandibleTextProps) => {
    const [isModalOpen, setIsModalOpen] = useState<boolean>(false);

    const isExpandible = text.length > maxLength;

    return (
        <>
            <div className="text-sm">
                <div className="whitespace-pre-wrap">
                    {isExpandible ? `${text.substring(0, maxLength)}...` : text}
                </div>
                {isExpandible && (
                    <div className="flex justify-end">
                        <button
                            onClick={() => setIsModalOpen(true)}
                            className="text-xs text-slate-500 flex items-center"
                        >
                            <Eye className="h-4 w-4 mr-1" />
                            Show more
                        </button>
                    </div>
                )}
            </div>

            <TextModal
                isOpen={isModalOpen}
                onClose={() => setIsModalOpen(false)}
                title="Sogc pubblication"
                text={text}
            />
        </>
    );
};