import { Settings, Info } from "lucide-react";

interface NoActionMessageProps {
    message: string;
}

export const NoActionMessage = ({ message }: NoActionMessageProps) => {
    return (
        <div className="space-y-3">
            <div className="flex items-center gap-2">
                <Settings className="h-4 w-4 text-slate-500" />
                <h4 className="font-medium">Action to Execute</h4>
            </div>

            <div className="bg-slate-50 rounded-lg p-4">
                <div className="flex items-start gap-2 p-2 bg-gray-50 rounded text-sm">
                    <Info className="h-4 w-4 text-gray-600 mt-0.5 flex-shrink-0" />
                    <span className="text-gray-700">
                        {message}
                    </span>
                </div>
            </div>
        </div>
    );
}
