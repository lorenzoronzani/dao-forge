import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";

interface TextModalProps {
    isOpen: boolean;
    onClose: () => void;
    text: string;
    title?: string;
}

export const TextModal = ({ isOpen, onClose, text, title = "Full Text" }: TextModalProps) => {
    return (
        <Dialog open={isOpen} onOpenChange={onClose}>
            <DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
                <DialogHeader>
                    <DialogTitle>{title}</DialogTitle>
                </DialogHeader>
                <div className="whitespace-pre-wrap py-4">
                    {text}
                </div>
            </DialogContent>
        </Dialog>
    );
};
