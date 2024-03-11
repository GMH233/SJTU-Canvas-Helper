import { Select, Space, Tooltip } from "antd";
import { Course } from "../lib/model";
import { InfoCircleOutlined } from '@ant-design/icons'

export default function CourseSelect({ courses, disabled, onChange }: {
    courses: Course[],
    disabled?: boolean,
    onChange?: (courseId: number) => void,
}) {
    const formatCourses = (courses: Course[]) => {
        const formatted: Course[] = [];
        courses.map(course => {
            const term = course.term.name.replace("Spring", "春").replace("Fall", "秋");
            formatted.push({
                ...course,
                name: `${course.name}(${term}, ${course.teachers[0].display_name})`
            });
        });
        return formatted;
    }

    const courseLabel = (course: Course) => {
        return course.enrollments.find(enrollment => enrollment.role === "TaEnrollment") ?
            <span><span style={{ color: "red" }}>*</span>{course.name}</span> :
            course.name
    }

    let formattedCourses = formatCourses(courses);

    return <Space>
        <span>选择课程：</span>
        <Select
            style={{ width: 350 }}
            disabled={disabled}
            onChange={onChange}
            options={formattedCourses.map(course => ({
                label: courseLabel(course),
                value: course.id
            }))}
        />
        <Tooltip placement="top" title={"带星号的课程为担任助教的课程"}>
            <InfoCircleOutlined />
        </Tooltip>
    </Space>
}