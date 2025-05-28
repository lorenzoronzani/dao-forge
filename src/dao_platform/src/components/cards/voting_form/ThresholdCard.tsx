import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { DateTimePicker } from "@/components/ui/date-time-picker";
import { CalendarClock } from "lucide-react";
import { Slider } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import { VerticalLabeledComponent } from "@/components/labels/VerticalLabeledComponent";

type ThresholdCardProps = {
    endAt: Date;
    approvalThreshold: number;
    quorum: number;
    onValueChange: (field: string, value: number | Date) => void;
}

export const ThresholdCard = ({ endAt, approvalThreshold, quorum, onValueChange }: ThresholdCardProps) => {
    return (
        <Card>
            <CardHeader>
                <CardTitle className="flex items-center gap-2">
                    <CalendarClock className="h-5 w-5" />
                    Timeline & Thresholds
                </CardTitle>
            </CardHeader>
            <CardContent className="space-y-6">
                <div>
                    <VerticalLabeledComponent label="End Date" htmlFor="end-date">
                        <DateTimePicker
                            date={endAt}
                            setDate={(date) => onValueChange('endAt', date)}
                            className="w-full"
                        />
                    </VerticalLabeledComponent>
                    <p className="text-xs text-slate-500 mt-1">The voting will automatically close at this time</p>
                </div>

                <div>
                    <div className="flex justify-between mb-1">
                        <Label htmlFor="approval">Approval Threshold</Label>
                        <span className="text-sm">{approvalThreshold}%</span>
                    </div>
                    <Slider
                        id="approval"
                        min={1}
                        max={100}
                        step={1}
                        value={[approvalThreshold]}
                        onValueChange={(values) => onValueChange('approvalThreshold', values[0])}
                    />
                    <p className="text-xs text-slate-500 mt-1">Percentage of votes needed for approval</p>
                </div>

                <div>
                    <div className="flex justify-between mb-1">
                        <Label htmlFor="quorum">Quorum</Label>
                        <span className="text-sm">{quorum}%</span>
                    </div>
                    <Slider
                        id="quorum"
                        min={1}
                        max={100}
                        step={1}
                        value={[quorum]}
                        onValueChange={(values) => onValueChange('quorum', values[0])}
                    />
                    <p className="text-xs text-slate-500 mt-1">Minimum participation required for valid results</p>
                </div>
            </CardContent>
        </Card>
    )
}

export default ThresholdCard;
